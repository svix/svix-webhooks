use self::config::Config;
use clap::Parser;
use lazy_static::lazy_static;
use opentelemetry::runtime::Tokio;
use opentelemetry_otlp::WithExportConfig;
use std::io::{Error, ErrorKind, Result};
use std::path::PathBuf;
use std::time::Duration;
use svix_bridge_types::{SenderInput, TransformerJob};
use svix_ksuid::{KsuidLike as _, KsuidMs};
use tracing_subscriber::prelude::*;

mod config;
mod runtime;
mod webhook_receiver;

lazy_static! {
    // Seems like it would be useful to be able to configure this.
    // In some docker setups, hostname is sometimes the container id, and advertising this can be
    // helpful.
    pub static ref INSTANCE_ID: String = KsuidMs::new(None, None).to_string();
}

fn get_svc_identifiers(cfg: &Config) -> opentelemetry::sdk::Resource {
    opentelemetry::sdk::Resource::new(vec![
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
    if std::env::var_os("RUST_LOG").is_none() {
        const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");
        let level = cfg.log_level.to_string();
        let var = vec![
            format!("{CRATE_NAME}={level}"),
            // XXX: Assuming this applies to the Producer side (aka `og-ingester`) when we fold it back in.
            format!("tower_http={level}"),
        ];
        std::env::set_var("RUST_LOG", var.join(","));
    }

    let otel_layer = cfg.opentelemetry.as_ref().map(|otel_cfg| {
        // Configure the OpenTelemetry tracing layer
        opentelemetry::global::set_text_map_propagator(
            opentelemetry::sdk::propagation::TraceContextPropagator::new(),
        );

        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(&otel_cfg.address);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(
                opentelemetry::sdk::trace::config()
                    .with_sampler(
                        otel_cfg
                            .sample_ratio
                            .map(opentelemetry::sdk::trace::Sampler::TraceIdRatioBased)
                            .unwrap_or(opentelemetry::sdk::trace::Sampler::AlwaysOn),
                    )
                    .with_resource(get_svc_identifiers(cfg)),
            )
            .install_batch(Tokio)
            .unwrap();

        tracing_opentelemetry::layer().with_tracer(tracer)
    });

    // Then initialize logging with an additional layer printing to stdout. This additional layer is
    // either formatted normally or in JSON format
    // Fails if the subscriber was already initialized, which we can safely and silently ignore.
    let _ = match cfg.log_format {
        config::LogFormat::Default => {
            let stdout_layer = tracing_subscriber::fmt::layer();
            tracing_subscriber::Registry::default()
                .with(otel_layer)
                .with(stdout_layer)
                .with(tracing_subscriber::EnvFilter::from_default_env())
                .try_init()
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
                .with(tracing_subscriber::EnvFilter::from_default_env())
                .try_init()
        }
    };
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
    #[arg(short, long, env = "SVIX_BRIDGE_CFG")]
    cfg: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config_path = args.cfg.unwrap_or_else(|| {
        std::env::current_dir()
            .expect("current dir")
            .join("svix-bridge.yaml")
    });

    let cfg_source = std::fs::read_to_string(&config_path).map_err(|e| {
        let p = config_path
            .into_os_string()
            .into_string()
            .expect("config path");
        Error::new(ErrorKind::Other, format!("Failed to read {p}: {e}"))
    })?;

    let vars = std::env::vars().collect();
    let cfg = Config::from_src(&cfg_source, Some(vars).as_ref())?;
    setup_tracing(&cfg);
    tracing::info!("starting");

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
        let tp = runtime::TpHandle::new();
        while let Some(TransformerJob {
            input,
            script,
            callback_tx,
        }) = xform_rx.recv().await
        {
            let tp = tp.clone();
            tokio::spawn(async move {
                let out = tp.run_script(input, script).await;
                if callback_tx
                    .send(out.map_err(|e| tracing::error!("{}", e)))
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
