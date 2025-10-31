// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use std::{borrow::Cow, sync::LazyLock, time::Duration};

use aide::axum::ApiRouter;
use cfg::ConfigurationInner;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{metrics::SdkMeterProvider, resource::ResourceDetector, runtime::Tokio};
use queue::TaskQueueProducer;
use redis::RedisManager;
use sea_orm::DatabaseConnection;
use sentry::integrations::tracing::EventFilter;
use svix_ksuid::{KsuidLike, KsuidMs};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tower::layer::layer_fn;
use tower_http::{
    cors::{AllowHeaders, Any, CorsLayer},
    normalize_path::NormalizePath,
};
use tracing_subscriber::{layer::SubscriberExt as _, Layer as _};

use crate::{
    cfg::{CacheBackend, Configuration},
    core::{
        cache,
        cache::Cache,
        idempotency::IdempotencyService,
        operational_webhooks::{OperationalWebhookSender, OperationalWebhookSenderInner},
    },
    db::init_db,
    expired_message_cleaner::expired_message_cleaner_loop,
    worker::queue_handler,
};

pub mod cfg;
pub mod core;
pub mod db;
pub mod error;
pub mod expired_message_cleaner;
pub mod metrics;
pub mod openapi;
pub mod queue;
pub mod redis;
pub mod v1;
pub mod worker;

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

static SHUTTING_DOWN_TOKEN: LazyLock<CancellationToken> = LazyLock::new(CancellationToken::new);

/// Has someone requested shutdown?
pub fn is_shutting_down() -> bool {
    SHUTTING_DOWN_TOKEN.is_cancelled()
}

/// Request a CancellationToken for the application shut down
pub fn shutting_down_token() -> CancellationToken {
    SHUTTING_DOWN_TOKEN.clone()
}

/// Shut down the application
pub fn start_shut_down() {
    SHUTTING_DOWN_TOKEN.cancel();
}

pub static INSTANCE_ID: LazyLock<String> =
    LazyLock::new(|| hex::encode(KsuidMs::new(None, None).to_string()));

async fn graceful_shutdown_handler() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = sigterm => {},
    }

    tracing::info!("Received shutdown signal. Shutting down gracefully...");
    start_shut_down();
}

pub async fn run(cfg: Configuration) {
    let _metrics = setup_metrics(&cfg);
    run_with_prefix(cfg.queue_prefix.clone(), cfg, None).await
}

#[derive(Clone)]
pub struct AppState {
    db: DatabaseConnection,
    queue_tx: TaskQueueProducer,
    cfg: Configuration,
    cache: Cache,
    op_webhooks: OperationalWebhookSender,
}

// Made public for the purpose of E2E testing in which a queue prefix is necessary to avoid tests
// consuming from each others' queues
pub async fn run_with_prefix(
    prefix: Option<String>,
    cfg: Configuration,
    listener: Option<TcpListener>,
) {
    tracing::debug!("DB: Initializing pool");
    let pool = init_db(&cfg).await;
    tracing::debug!("DB: Started");

    tracing::debug!("Cache: Initializing {:?}", cfg.cache_type);
    let cache_backend = cfg.cache_backend();
    let cache = match &cache_backend {
        CacheBackend::None => cache::none::new(),
        CacheBackend::Memory => cache::memory::new(),
        CacheBackend::Redis(_)
        | CacheBackend::RedisCluster(_)
        | CacheBackend::RedisSentinel(_, _) => {
            let mgr = RedisManager::from_cache_backend(&cache_backend).await;
            cache::redis::new(mgr)
        }
    };
    tracing::debug!("Cache: Started");

    tracing::debug!("Queue: Initializing {:?}", cfg.queue_type);
    let (queue_tx, queue_rx) = queue::new_pair(&cfg, prefix.as_deref()).await;
    tracing::debug!("Queue: Started");

    let op_webhook_sender = OperationalWebhookSenderInner::new(
        cfg.jwt_signing_config.clone(),
        cfg.operational_webhook_address.clone(),
    );

    // OpenAPI/aide must be initialized before any routers are constructed
    // because its initialization sets generation-global settings which are
    // needed at router-construction time.
    let mut openapi = openapi::initialize_openapi();

    let svc_cache = cache.clone();
    // build our application with a route
    let app_state = AppState {
        db: pool.clone(),
        queue_tx: queue_tx.clone(),
        cfg: cfg.clone(),
        cache: cache.clone(),
        op_webhooks: op_webhook_sender.clone(),
    };
    let v1_router = v1::router().with_state::<()>(app_state);

    // Initialize all routes which need to be part of OpenAPI first.
    let app = ApiRouter::new()
        .nest_api_service("/api/v1", v1_router)
        .finish_api(&mut openapi);

    openapi::postprocess_spec(&mut openapi);
    let docs_router = docs::router(openapi);
    let app = app.merge(docs_router).layer((
        layer_fn(move |service| IdempotencyService {
            cache: svc_cache.clone(),
            service,
        }),
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(AllowHeaders::mirror_request())
            .max_age(Duration::from_secs(600)),
    ));
    let svc = tower::make::Shared::new(
        // It is important that this service wraps the router instead of being
        // applied via `Router::layer`, as it would run after routing then.
        NormalizePath::trim_trailing_slash(app),
    );

    let with_api = cfg.api_enabled;
    let with_worker = cfg.worker_enabled;
    let listen_address = cfg.listen_address;

    let ((), worker_loop, expired_message_cleaner_loop) = tokio::join!(
        async {
            if with_api {
                let listener = match listener {
                    Some(l) => l,
                    None => TcpListener::bind(listen_address)
                        .await
                        .expect("Error binding to listen_address"),
                };
                tracing::debug!("API: Listening on {}", listener.local_addr().unwrap());

                axum::serve(listener, svc)
                    .with_graceful_shutdown(graceful_shutdown_handler())
                    .await
                    .unwrap();
            } else {
                tracing::debug!("API: off");
                graceful_shutdown_handler().await;
            }
        },
        async {
            if with_worker {
                tracing::debug!("Worker: Started");
                queue_handler(
                    &cfg,
                    cache.clone(),
                    pool.clone(),
                    queue_tx,
                    queue_rx,
                    op_webhook_sender,
                )
                .await
            } else {
                tracing::debug!("Worker: off");
                Ok(())
            }
        },
        async {
            if with_worker {
                tracing::debug!("Expired message cleaner: Started");
                expired_message_cleaner_loop(&pool).await
            } else {
                tracing::debug!("Expired message cleaner: off");
                Ok(())
            }
        }
    );

    worker_loop.expect("Error initializing worker");
    expired_message_cleaner_loop.expect("Error initializing expired message cleaner")
}

pub fn setup_tracing(
    cfg: &ConfigurationInner,
    for_test: bool,
) -> (tracing::Dispatch, sentry::ClientInitGuard) {
    let filter_directives = std::env::var("RUST_LOG").unwrap_or_else(|e| {
        if let std::env::VarError::NotUnicode(_) = e {
            eprintln!("RUST_LOG environment variable has non-utf8 contents, ignoring!");
        }

        let level = cfg.log_level.to_string();
        let mut var = vec![
            format!("{CRATE_NAME}={level}"),
            format!("tower_http={level}"),
        ];

        if cfg.db_tracing {
            var.push(format!("sqlx={level}"));
        }

        var.join(",")
    });

    let otel_layer = cfg.opentelemetry_address.as_ref().map(|addr| {
        // Configure the OpenTelemetry tracing layer
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(addr);

        let resource_detector = opentelemetry_sdk::resource::EnvResourceDetector::new();
        let resource = resource_detector
            .detect(std::time::Duration::from_secs(5))
            .merge(&opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new(
                    "service.name",
                    cfg.opentelemetry_service_name.clone(),
                ),
            ]));

        let provider = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(
                opentelemetry_sdk::trace::Config::default()
                    .with_sampler(
                        cfg.opentelemetry_sample_ratio
                            .map(opentelemetry_sdk::trace::Sampler::TraceIdRatioBased)
                            .unwrap_or(opentelemetry_sdk::trace::Sampler::AlwaysOn),
                    )
                    .with_resource(resource),
            )
            .install_batch(Tokio)
            .unwrap();

        // Based on the private `build_batch_with_exporter` method from opentelemetry-otlp
        let tracer = provider
            .tracer_builder("opentelemetry-otlp")
            .with_schema_url(opentelemetry_semantic_conventions::SCHEMA_URL)
            .build();

        tracing_opentelemetry::layer().with_tracer(tracer)
    });

    let sentry_guard = sentry::init(sentry::ClientOptions {
        dsn: cfg.sentry_dsn.clone(),
        environment: Some(Cow::Owned(cfg.environment.to_string())),
        release: sentry::release_name!(),
        ..Default::default()
    });

    let sentry_layer =
        sentry::integrations::tracing::layer().event_filter(|md| match *md.level() {
            tracing::Level::ERROR | tracing::Level::WARN => EventFilter::Event,
            _ => EventFilter::Ignore,
        });

    // Then create a subscriber with an additional layer printing to stdout.
    // This additional layer is either formatted normally or in JSON format.
    let stdout_layer = if for_test {
        tracing_subscriber::fmt::layer().with_test_writer().boxed()
    } else {
        match cfg.log_format {
            cfg::LogFormat::Default => tracing_subscriber::fmt::layer().boxed(),
            cfg::LogFormat::Json => {
                let fmt = tracing_subscriber::fmt::format().json().flatten_event(true);
                let json_fields = tracing_subscriber::fmt::format::JsonFields::new();

                tracing_subscriber::fmt::layer()
                    .event_format(fmt)
                    .fmt_fields(json_fields)
                    .boxed()
            }
        }
    };

    let registry = tracing_subscriber::Registry::default()
        .with(otel_layer)
        .with(sentry_layer)
        .with(stdout_layer)
        .with(tracing_subscriber::EnvFilter::new(filter_directives))
        .into();

    (registry, sentry_guard)
}

pub fn setup_metrics(cfg: &ConfigurationInner) -> Option<SdkMeterProvider> {
    cfg.opentelemetry_address.as_ref().map(|addr| {
        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(addr);

        opentelemetry_otlp::new_pipeline()
            .metrics(Tokio)
            .with_delta_temporality()
            .with_exporter(exporter)
            .with_resource(opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new(
                    "service.name",
                    cfg.opentelemetry_service_name.clone(),
                ),
                opentelemetry::KeyValue::new("instance_id", INSTANCE_ID.to_owned()),
                opentelemetry::KeyValue::new(
                    "service.version",
                    option_env!("GITHUB_SHA").unwrap_or("unknown"),
                ),
            ]))
            .build()
            .unwrap()
    })
}

pub fn setup_tracing_for_tests() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Output is only printed for failing tests, but still we shouldn't overload
                // the output with unnecessary info. When debugging a specific test, it's easy
                // to override this default by setting the `RUST_LOG` environment variable.
                "svix_server=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .init();
}

#[cfg(test)]
#[ctor::ctor]
fn test_setup() {
    setup_tracing_for_tests();
}

mod docs {
    use aide::{axum::ApiRouter, openapi::OpenApi};
    use axum::{
        response::{Html, IntoResponse, Redirect},
        routing::get,
    };

    // TODO: switch to generated docs instead of hardcoded JSON once generated
    // is comparable/better than hardcoded one.
    pub fn router(_docs: OpenApi) -> ApiRouter {
        ApiRouter::new()
            .route("/", get(|| async { Redirect::temporary("/docs") }))
            .route("/docs", get(get_docs))
            .route("/api/v1/openapi.json", get(get_openapi_json))
            .with_state(_docs)
    }

    async fn get_docs() -> Html<&'static str> {
        Html(include_str!("static/docs.html"))
    }

    async fn get_openapi_json() -> impl IntoResponse {
        static BODY: &str = include_str!("../../openapi.json");
        ([(http::header::CONTENT_TYPE, "application/json")], BODY)
    }
}
