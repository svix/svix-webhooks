// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use dotenv::dotenv;
use opentelemetry::runtime::Tokio;
use opentelemetry_otlp::WithExportConfig;
use std::process::exit;
use svix_server::cfg::DefaultSignatureType;
use svix_server::core::types::{EndpointSecretInternal, EndpointSecretType, OrganizationId};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use validator::Validate;

use svix_server::core::security::{default_org_id, generate_org_token};

use svix_server::{cfg, db, run};

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

use clap::{Parser, Subcommand};

mod wait_for;
use wait_for::wait_for_dsn;

// The names and default ports of services to wait-for
const POSTGRES_NAME: &str = "PostgreSQL";
const POSTGRES_PORT: u16 = 5432;

const REDIS_NAME: &str = "Redis";
const REDIS_PORT: u16 = 6379;

#[derive(Parser)]
#[clap(author, version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// Run database migrations before starting
    #[clap(long, value_parser)]
    run_migrations: bool,

    /// The time to wait for a successful connection to the database before timing out in seconds.
    #[clap(long, value_parser)]
    wait_for: Option<u64>,
}

#[derive(Subcommand)]
enum Commands {
    /// JWT utilities
    #[clap()]
    Jwt {
        #[clap(subcommand)]
        command: JwtCommands,
    },
    /// Asymmetric Key utilities
    #[clap()]
    AsymmetricKey {
        #[clap(subcommand)]
        command: AsymmetricKeyCommands,
    },
    /// Run database migrations and exit
    #[clap()]
    Migrate,
}

#[derive(Subcommand)]
enum JwtCommands {
    /// Generate a new JWT
    #[clap()]
    Generate {
        #[clap(value_parser = org_id_parser)]
        /// Optional org_id to use when generating token (otherwise, default is used).
        org_id: Option<OrganizationId>,
    },
}

#[derive(Subcommand)]
enum AsymmetricKeyCommands {
    /// Generate a new asymmetric key
    #[clap()]
    Generate {
        #[clap(value_parser = signature_type_parser)]
        /// Optional type to use when generating a secret
        type_: Option<DefaultSignatureType>,
    },
}

fn signature_type_parser(s: &str) -> Result<DefaultSignatureType, String> {
    // XXX A bit hacky, but it's fine since an issue will just fail serde immediately after
    let type_ = format!("\"{}\"", s);
    serde_json::from_str(&type_).map_err(|x| x.to_string())
}

fn org_id_parser(s: &str) -> Result<OrganizationId, String> {
    let ret = OrganizationId(s.to_owned());
    ret.validate().map_err(|x| x.to_string())?;
    Ok(ret)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();
    let cfg = cfg::load().expect("Error loading configuration");

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{crate}={level},tower_http={level}",
                crate = CRATE_NAME,
                level = cfg.log_level.to_string()
            ),
        );
    }

    let otel_layer = cfg.opentelemetry_address.as_ref().map(|addr| {
        // Configure the OpenTelemetry tracing layer
        opentelemetry::global::set_text_map_propagator(
            opentelemetry::sdk::propagation::TraceContextPropagator::new(),
        );

        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(addr);

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(
                opentelemetry::sdk::trace::config()
                    .with_sampler(
                        cfg.opentelemetry_sample_ratio
                            .map(opentelemetry::sdk::trace::Sampler::TraceIdRatioBased)
                            .unwrap_or(opentelemetry::sdk::trace::Sampler::AlwaysOn),
                    )
                    .with_resource(opentelemetry::sdk::Resource::new(vec![
                        opentelemetry::KeyValue::new("service.name", "svix_server"),
                    ])),
            )
            .install_batch(Tokio)
            .unwrap();
        tracing_opentelemetry::layer().with_tracer(tracer)
    });

    // Then initialize logging with an additional layer priting to stdout. This additional layer is
    // either formatted normally or in JSON format
    match cfg.log_format {
        cfg::LogFormat::Default => {
            let stdout_layer = tracing_subscriber::fmt::layer();
            tracing_subscriber::Registry::default()
                .with(otel_layer)
                .with(stdout_layer)
                .with(tracing_subscriber::EnvFilter::from_default_env())
                .init();
        }
        cfg::LogFormat::Json => {
            let fmt = tracing_subscriber::fmt::format().json().flatten_event(true);
            let json_fields = tracing_subscriber::fmt::format::JsonFields::new();

            let stdout_layer = tracing_subscriber::fmt::layer()
                .event_format(fmt)
                .fmt_fields(json_fields);

            tracing_subscriber::Registry::default()
                .with(otel_layer)
                .with(stdout_layer)
                .with(tracing_subscriber::EnvFilter::from_default_env())
                .init();
        }
    };

    if let Some(wait_for_seconds) = args.wait_for {
        let mut wait_for = Vec::with_capacity(2);
        wait_for.push(wait_for_dsn(
            &cfg.db_dsn,
            POSTGRES_NAME,
            POSTGRES_PORT,
            wait_for_seconds,
        ));

        if let Some(redis_dsn) = &cfg.redis_dsn {
            wait_for.push(wait_for_dsn(
                redis_dsn,
                REDIS_NAME,
                REDIS_PORT,
                wait_for_seconds,
            ));
        }

        futures::future::join_all(wait_for).await;
    }

    if args.run_migrations {
        db::run_migrations(&cfg).await;
        tracing::debug!("Migrations: success");
    }

    match args.command {
        Some(Commands::Migrate) => {
            db::run_migrations(&cfg).await;
            println!("Migrations: success");
            exit(0);
        }
        Some(Commands::Jwt {
            command: JwtCommands::Generate { org_id },
        }) => {
            let org_id = org_id.unwrap_or_else(default_org_id);
            let token =
                generate_org_token(&cfg.jwt_secret, org_id).expect("Error generating token");
            println!("Token (Bearer): {}", token);
            exit(0);
        }
        Some(Commands::AsymmetricKey { command }) => match command {
            AsymmetricKeyCommands::Generate { type_ } => {
                let secret = match type_ {
                    Some(type_) => EndpointSecretInternal::generate(&cfg.encryption, type_.into()),
                    None => EndpointSecretInternal::generate(
                        &cfg.encryption,
                        EndpointSecretType::Ed25519,
                    ),
                }
                .unwrap()
                .into_endpoint_secret(&cfg.encryption)
                .unwrap();
                println!("Secret key: {}", secret.serialize_secret_key());
                println!("Public key: {}", secret.serialize_public_key());
                exit(0);
            }
        },
        None => {}
    };

    run(cfg, None).await;

    opentelemetry::global::shutdown_tracer_provider();
}
