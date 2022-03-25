// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use axum::{extract::Extension, Router};
use bb8_redis::RedisConnectionManager;
use cfg::QueueType;
use dotenv::dotenv;
use std::{
    net::{SocketAddr, TcpListener},
    process::exit,
    str::FromStr,
};
use tower_http::trace::TraceLayer;

use crate::{
    cfg::Configuration,
    core::{
        cache::RedisCache,
        security::{default_org_id, generate_token},
    },
    db::init_db,
    worker::worker_loop,
};

mod cfg;
mod core;
mod db;
mod error;
mod queue;
mod v1;
mod worker;

#[cfg(test)]
pub(crate) mod test_util;

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

pub(crate) async fn run(cfg: Configuration, listener: Option<TcpListener>) {
    let pool = init_db(&cfg).await;
    let redis_pool = if let Some(redis_dsn) = &cfg.redis_dsn {
        tracing::debug!("Redis: Initializing pool");
        let manager = RedisConnectionManager::new(redis_dsn.to_string()).unwrap();
        Some(bb8::Pool::builder().build(manager).await.unwrap())
    } else {
        None
    };

    tracing::debug!("Queue type: {:?}", cfg.queue_type);
    let (queue_tx, queue_rx) = match cfg.queue_type {
        QueueType::Memory => queue::memory::new_pair().await,
        QueueType::Redis => {
            queue::redis::new_pair(redis_pool.clone().unwrap_or_else(|| {
                panic!("Choosing queue_type=Redis requires setting a redis DSN.")
            }))
            .await
        }
    };

    let redis_cache = redis_pool
        .as_ref()
        .map(|pool| RedisCache::new(pool.clone()));

    // build our application with a route
    let mut app = Router::new()
        .nest("/api/v1", v1::router())
        .layer(TraceLayer::new_for_http().on_request(()))
        .layer(Extension(pool.clone()))
        .layer(Extension(queue_tx.clone()))
        .layer(Extension(cfg.clone()))
        .layer(Extension(redis_cache.clone()));

    if let Some(redis_pool) = &redis_pool {
        app = app.layer(Extension(redis_pool.clone()));
    };

    let with_api = cfg.api_enabled;
    let with_worker = cfg.worker_enabled;

    let listen_address = SocketAddr::from_str(&cfg.listen_address).unwrap();

    let (server, worker_loop) = tokio::join!(
        async {
            if with_api {
                if let Some(l) = listener {
                    tracing::debug!("API: Listening on {}", l.local_addr().unwrap());
                    axum::Server::from_tcp(l)
                        .unwrap()
                        .serve(app.into_make_service())
                        .await
                } else {
                    tracing::debug!("API: Listening on {}", listen_address);
                    axum::Server::bind(&listen_address)
                        .serve(app.into_make_service())
                        .await
                }
            } else {
                tracing::debug!("API: off");
                Ok(())
            }
        },
        async {
            if with_worker {
                tracing::debug!("Worker: Initializing");
                worker_loop(cfg, pool, redis_cache, queue_tx, queue_rx).await
            } else {
                tracing::debug!("Worker: off");
                Ok(())
            }
        },
    );
    server.unwrap();
    worker_loop.unwrap();
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = Args::parse();
    let cfg = cfg::load().unwrap();

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
        let token = generate_token(&cfg.jwt_secret, default_org_id()).unwrap();
        println!("Token (Bearer): {}", token);
        exit(0);
    }

    run(cfg, None).await;
}
