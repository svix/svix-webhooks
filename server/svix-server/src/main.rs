// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use dotenv::dotenv;
use trust_dns_resolver::TokioHandle;
use std::net::IpAddr;
use std::{process::exit, time::Duration};
use svix_server::core::types::OrganizationId;
use trust_dns_resolver::{config::{ResolverConfig, ResolverOpts}, TokioAsyncResolver};
use validator::Validate;

use svix_server::core::security::{default_org_id, generate_org_token};

use svix_server::{cfg, db, run};

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

use clap::{Parser, Subcommand};

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

    match cfg.log_format {
        cfg::LogFormat::Default => {
            tracing_subscriber::fmt::init();
        }
        cfg::LogFormat::Json => {
            let fmt = tracing_subscriber::fmt::format().json().flatten_event(true);
            let json_fields = tracing_subscriber::fmt::format::JsonFields::new();
            let filter = tracing_subscriber::EnvFilter::from_default_env();

            tracing_subscriber::fmt()
                .event_format(fmt)
                .fmt_fields(json_fields)
                .with_env_filter(filter)
                .init();
        }
    };

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
        None => {}
    };

    if let Some(wait_for_seconds) = args.wait_for {
        let pg = &cfg.db_dsn;

        let pg = url::Url::parse(pg).expect("Invalid PostgreSQL DNS");

        let pg_host = pg.host().expect("Expected PostgreSQL host");
        let pg_port = pg.port().unwrap_or(5432);

        let pg_host: IpAddr = match pg_host {
            url::Host::Domain(s) => {
                let resolver = TokioAsyncResolver::new(
                    ResolverConfig::default(), ResolverOpts::default(), TokioHandle)
                    .expect("Could not set up DNS resolution");

                let resp = resolver.lookup_ip(s).await.expect("DNS error");
                resp.iter()
                    .next()
                    .expect("Expected at least one IP address")
            }
            url::Host::Ipv4(ipv4_addr) => ipv4_addr.into(),
            url::Host::Ipv6(ipv6_addr) => ipv6_addr.into(),
        };

        let sleep = tokio::time::sleep(Duration::from_secs(wait_for_seconds));
        tokio::pin!(sleep);
        loop {
            tokio::select! {
                _ = tokio::net::TcpStream::connect((pg_host, pg_port)) => {
                    break;
                },
                _ = &mut sleep => {
                    panic!("Waiting for databases timed out");
                }
            }
        }

        let redis = cfg.redis_dsn.as_deref();

        if let Some(redis) = redis {
            let redis = url::Url::parse(redis).expect("Invalid Redis DSN");

            let redis_host = redis.host().expect("Exepected Redis host");
            let redis_port = redis.port().unwrap_or(6379);

            let redis_host: IpAddr = match redis_host {
                url::Host::Domain(s) => {
                let resolver = TokioAsyncResolver::new(
                    ResolverConfig::default(), ResolverOpts::default(), TokioHandle)
                    .expect("Could not set up DNS resolution");

                let resp = resolver.lookup_ip(s).await.expect("DNS error");
                resp.iter()
                    .next()
                    .expect("Expected at least one IP address")

                }
                url::Host::Ipv4(ipv4_addr) => ipv4_addr.into(),
                url::Host::Ipv6(ipv6_addr) => ipv6_addr.into(),
            };

            let sleep = tokio::time::sleep(Duration::from_secs(wait_for_seconds));
            tokio::pin!(sleep);
            loop {
                tokio::select! {
                    _ = tokio::net::TcpStream::connect((redis_host, redis_port)) => {
                        break;
                    },
                    _ = &mut sleep => {
                        panic!("Waiting for databases timed out");
                    }
                }
            }
        }
    }

    run(cfg, None).await;
}
