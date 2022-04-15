// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use axum::{extract::Extension, Router};
use cfg::QueueType;
use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
};
use tower_http::trace::TraceLayer;

use crate::{
    cfg::Configuration, core::cache::RedisCache, db::init_db, redis::RedisClusterConnectionManager,
    worker::worker_loop,
};

pub mod cfg;
pub mod core;
pub mod db;
pub mod error;
pub mod queue;
pub mod redis;
pub mod v1;
pub mod webhook;
pub mod worker;

pub async fn run(cfg: Configuration, listener: Option<TcpListener>) {
    let pool = init_db(&cfg).await;
    let redis_pool = if let Some(redis_dsn) = &cfg.redis_dsn {
        tracing::debug!("Redis: Initializing pool {}", redis_dsn);
        // let manager = RedisConnectionManager::new(redis_dsn.to_string()).unwrap();
        let manager = RedisClusterConnectionManager::new(vec![redis_dsn.to_string()]).unwrap();
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
