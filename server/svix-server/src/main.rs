// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use dotenv::dotenv;
use std::process::exit;

use svix_server::core::security::{default_org_id, generate_token};

use svix_server::{cfg, db, run};

const CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// JWT utilities
    #[clap()]
    Jwt {
        #[clap(subcommand)]
        command: JwtCommands,
    },

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

    if cfg!(debug_assertions) && std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{crate}={level},tower_http={level}",
                crate = CRATE_NAME,
                level = cfg.log_level.to_string()
            ),
        );
    }

    tracing_subscriber::fmt::init();

    if let Some(Commands::Migrate) = &args.command {
        let _ = db::init_db_and_run_migrations(&cfg).await;
        println!("Migrations run");
        exit(0);
    } else if let Some(Commands::Jwt {
        command: JwtCommands::Generate,
    }) = &args.command
    {
        let token =
            generate_token(&cfg.jwt_secret, default_org_id()).expect("Error generating token");
        println!("Token (Bearer): {}", token);
        exit(0);
    }

    run(cfg, None).await;
}
