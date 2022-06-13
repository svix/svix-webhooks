// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use dotenv::dotenv;
use std::process::exit;

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
    Generate,
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
        println!("Migrations run");
    }

    if let Some(Commands::Migrate) = &args.command {
        db::run_migrations(&cfg).await;
        println!("Migrations run");
        exit(0);
    } else if let Some(Commands::Jwt {
        command: JwtCommands::Generate,
    }) = &args.command
    {
        let token =
            generate_org_token(&cfg.jwt_secret, default_org_id()).expect("Error generating token");
        println!("Token (Bearer): {}", token);
        exit(0);
    }

    run(cfg, None).await;
}
