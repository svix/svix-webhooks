use std::{
    io::{Error, ErrorKind, Result},
    path::PathBuf,
    time::Duration,
};

use clap::Parser;
use once_cell::sync::Lazy;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    metrics::{data::Temporality, reader::TemporalitySelector, InstrumentKind, SdkMeterProvider},
    runtime::Tokio,
};
use svix_bridge_types::{SenderInput, TransformerJob};
use svix_ksuid::{KsuidLike as _, KsuidMs};
#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
use tikv_jemallocator::Jemalloc;
use tracing::Instrument;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

use self::config::Config;

mod allocator;
mod config;
mod metrics;
mod runtime;
mod webhook_receiver;

use crate::{
    allocator::{get_allocator_stat_mibs, get_allocator_stats},
    metrics::CommonMetrics,
};

#[cfg(all(not(target_env = "msvc"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[cfg(all(target_env = "msvc", feature = "jemalloc"))]
compile_error!("jemalloc cannot be enabled on msvc");

// Seems like it would be useful to be able to configure this.
// In some docker setups, hostname is sometimes the container id, and advertising this can be
// helpful.
static INSTANCE_ID: Lazy<String> = Lazy::new(|| KsuidMs::new(None, None).to_string());

fn get_svc_identifiers(cfg: &Config) -> opentelemetry_sdk::Resource {
    opentelemetry_sdk::Resource::new(vec![
        opentelemetry::KeyValue::new(
            "service.name",
            cfg.opentelemetry
                .as_ref()
                .and_then(|x| x.service_name.as_deref())
                .unwrap_or("svix-bridge")
                .to_owned(),
        ),
        opentelemetry::KeyValue::new("instance_id", INSTANCE_ID.to_owned()),
    ])
}

fn setup_tracing(cfg: &Config) {
    let filter_directives = std::env::var("RUST_LOG").unwrap_or_else(|e| {
        if let std::env::VarError::NotUnicode(_) = e {
            eprintln!("RUST_LOG environment variable has non-utf8 contents, ignoring!");
        }

        const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");
        let level = cfg.log_level.to_string();
        let var = [
            format!("{CRATE_NAME}={level}"),
            // XXX: Assuming this applies to the Producer side (aka `og-ingester`) when we fold it back in.
            format!("tower_http={level}"),
        ];
        var.join(",")
    });

    let otel_layer = cfg.opentelemetry.as_ref().map(|otel_cfg| {
        // Configure the OpenTelemetry tracing layer
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(&otel_cfg.address);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(
                opentelemetry_sdk::trace::config()
                    .with_sampler(
                        otel_cfg
                            .sample_ratio
                            .map(opentelemetry_sdk::trace::Sampler::TraceIdRatioBased)
                            .unwrap_or(opentelemetry_sdk::trace::Sampler::AlwaysOn),
                    )
                    .with_resource(get_svc_identifiers(cfg)),
            )
            .install_batch(Tokio)
            .unwrap();

        tracing_opentelemetry::layer().with_tracer(tracer)
    });

    // Then create a subscriber with an additional layer printing to stdout.
    // This additional layer is either formatted normally or in JSON format.
    match cfg.log_format {
        config::LogFormat::Default => {
            let stdout_layer = tracing_subscriber::fmt::layer();
            tracing_subscriber::Registry::default()
                .with(otel_layer)
                .with(stdout_layer)
                .with(tracing_subscriber::EnvFilter::new(filter_directives))
                .init()
        }
        config::LogFormat::Json => {
            let fmt = tracing_subscriber::fmt::format().json().flatten_event(true);
            let json_fields = tracing_subscriber::fmt::format::JsonFields::new();

            let stdout_layer = tracing_subscriber::fmt::layer()
                .event_format(fmt)
                .fmt_fields(json_fields);

            tracing_subscriber::Registry::default()
                .with(otel_layer)
                .with(stdout_layer)
                .with(tracing_subscriber::EnvFilter::new(filter_directives))
                .init()
        }
    };
}

/// Delta temporality selector as recommended by upstream:
/// https://github.com/open-telemetry/opentelemetry-rust/discussions/1511#discussioncomment-8386721
struct DeltaTemporalitySelector;

impl TemporalitySelector for DeltaTemporalitySelector {
    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        match kind {
            InstrumentKind::UpDownCounter => Temporality::Cumulative,
            InstrumentKind::ObservableUpDownCounter => Temporality::Cumulative,
            _ => Temporality::Delta,
        }
    }
}

pub fn setup_metrics(cfg: &Config) -> Option<SdkMeterProvider> {
    cfg.opentelemetry.as_ref().map(|otel_cfg| {
        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(&otel_cfg.address);

        opentelemetry_otlp::new_pipeline()
            .metrics(Tokio)
            .with_temporality_selector(DeltaTemporalitySelector)
            .with_exporter(exporter)
            .with_resource(get_svc_identifiers(cfg))
            .build()
            .unwrap()
    })
}

async fn supervise_senders(inputs: Vec<Box<dyn SenderInput>>) -> Result<()> {
    let mut set = tokio::task::JoinSet::new();
    for input in inputs {
        set.spawn(async move {
            // FIXME: needs much better signaling for termination
            loop {
                let fut = input.run();
                // If this future returns, the consumer terminated unexpectedly.
                if let Err(e) = fut.await {
                    tracing::warn!(
                        "sender input {} unexpectedly terminated: {}",
                        input.name(),
                        e
                    );
                } else {
                    tracing::warn!("sender input {} unexpectedly terminated", input.name());
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
    }

    // FIXME: add signal handling to trigger a (intentional) graceful shutdown.

    // FIXME: when a plugin exits unexpectedly, what do?
    //   Most consumers are probably stateful/brittle and may disconnect from time to time.
    //   Ideally none of these tasks would ever return Ok or Err. They'd run forever.
    //   Having the tasks themselves try to recover means if we see a task finish here, something
    //   must be really wrong, so maybe we trigger a shutdown of the rest when one stops here.
    while let Some(_res) = set.join_next().await {
        // In order for plugins to coordinate a shutdown, maybe they could:
        // - have a shutdown method and handle their own internal signalling, or maybe
        // - take a oneshot channel as an arg to `run()`
        // Basically we need something that formalizes the shutdown flow in a cross-crate
        // friendly way.
        todo!("graceful shutdown");
    }
    Ok(())
}

#[derive(Parser)]
pub struct Args {
    #[arg(long, env = "SVIX_BRIDGE_CFG_FILE", help = "Path to the config file.")]
    cfg_file: Option<PathBuf>,
    #[arg(
        long,
        env = "SVIX_BRIDGE_CFG",
        help = "Config data as a string (instead of a file on disk).",
        conflicts_with = "cfg_file"
    )]
    cfg: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut config_search_paths = vec![];

    if let Some(fp) = args.cfg_file {
        config_search_paths.push(fp)
    } else {
        for name in ["svix-bridge.yaml", "svix-bridge.yml", "svix-bridge.json"] {
            config_search_paths.push(std::env::current_dir().expect("current dir").join(name));
        }
    }

    // Clap will ensure we have only one or the other (cfg and cfg_file can't be specified together).
    let cfg_source = match args.cfg {
        Some(cfg_source) => cfg_source,
        None => {
            let fp = config_search_paths
                .into_iter()
                .find(|x| x.exists())
                .expect("config file path");
            std::fs::read_to_string(&fp).map_err(|e| {
                let p = fp.into_os_string().into_string().expect("config file path");
                Error::new(ErrorKind::Other, format!("Failed to read {p}: {e}"))
            })
        }?,
    };

    let vars = std::env::vars().collect();
    let cfg = Config::from_src(&cfg_source, Some(vars).as_ref())?;
    setup_tracing(&cfg);
    let _metrics = setup_metrics(&cfg);
    tracing::info!("starting");

    tokio::spawn(
        async move {
            let mut interval = tokio::time::interval(Duration::from_secs(15));
            let metrics = CommonMetrics::new(&opentelemetry::global::meter("svix.com"));
            match get_allocator_stat_mibs() {
                Ok(mibs) => {
                    tracing::debug!("Common Metrics Collection: Started");

                    loop {
                        interval.tick().await;

                        if let Ok(Some((allocated, resident))) =
                            get_allocator_stats(true, mibs.clone())
                        {
                            metrics.record_mem_allocated(allocated as _);
                            metrics.record_mem_resident(resident as _);
                        }
                    }
                }
                Err(e) => tracing::error!("Unable to get allocator stats mibs: {e}"),
            }
        }
        .instrument(tracing::error_span!(
            "common_metrics_collector",
            instance_id = tracing::field::Empty
        )),
    );

    let (xform_tx, mut xform_rx) = tokio::sync::mpsc::unbounded_channel::<TransformerJob>();

    // XXX: this is a bit nasty, but might be okay to start.
    // The nested spawns are needed to make sure we can saturate the
    // threadpool (otherwise we'd run each job serially).
    //
    // Another approach would be to do what og-ingester did: give each plugin a clone of the
    // `TpHandle`, but this would likely mean moving the runtime module over to the `-types` crate.
    // I'd rather not do this, mostly to help keep things more unit test friendly; channels can
    // help keep the coupling more loose, with less stateful baggage.
    // Starting with this just to keep the JS executor stuff here in the binary.
    tokio::spawn(async move {
        tracing::info!(
            "Starting JS Transformation Workers: {}",
            cfg.transformation_worker_count
        );
        let pooler = runtime::JsPooler::new(cfg.transformation_worker_count);
        while let Some(TransformerJob {
            input,
            script,
            callback_tx,
        }) = xform_rx.recv().await
        {
            let tp = pooler.clone();
            tokio::spawn(async move {
                let out = tp.run_script(input, script).await;
                // FIXME: seeing this Err case come up during load testing.
                //   Seems like we shouldn't be hitting this so easily while the process is not terminating.
                //   Regularly there are group error log lines that show up right at the end of an
                //   `oha` run, POSTing to receivers. Need to investigate why.
                if callback_tx
                    .send(out.map_err(|e| tracing::error!("{:?}", e)))
                    .is_err()
                {
                    // If the callback fails, the plugin is likely unwinding/dropping.
                    // Not a whole lot we can do about that.
                    tracing::error!("failed to send js output back to caller");
                }
            });
        }
    });

    let mut senders = Vec::with_capacity(cfg.senders.len());
    for sc in cfg.senders {
        let mut sender: Box<dyn SenderInput> =
            sc.try_into().map_err(|e| Error::new(ErrorKind::Other, e))?;
        sender.set_transformer(Some(xform_tx.clone()));
        senders.push(sender);
    }
    if senders.is_empty() {
        tracing::warn!("No senders configured.")
    }
    let senders_fut = supervise_senders(senders);

    if cfg.receivers.is_empty() {
        tracing::warn!("No receivers configured.")
    }
    let receivers_fut = webhook_receiver::run(cfg.http_listen_address, cfg.receivers, xform_tx);

    match tokio::try_join!(senders_fut, receivers_fut) {
        Ok(_) => tracing::error!("unexpectedly exiting"),
        Err(e) => tracing::error!("unexpectedly exiting: {}", e),
    }

    tracing::info!("exiting...");
    Ok(())
}
