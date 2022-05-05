// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use axum::{extract::Extension, Router};

use cfg::{CacheType, QueueType};
use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{
    cfg::Configuration,
    core::{cache, idempotency::IdempotencyService},
    db::init_db,
    expired_message_cleaner::expired_message_cleaner_loop,
    worker::worker_loop,
};

pub mod cfg;
pub mod core;
pub mod db;
pub mod error;
pub mod expired_message_cleaner;
pub mod queue;
pub mod redis;
pub mod v1;
pub mod webhook;
pub mod worker;

pub async fn run(cfg: Configuration, listener: Option<TcpListener>) {
    let pool = init_db(&cfg).await;

    let redis_dsn = || {
        cfg.redis_dsn
            .as_ref()
            .expect("Redis DSN not found")
            .as_str()
    };

    tracing::debug!("Cache type: {:?}", cfg.cache_type);
    let cache = match cfg.cache_type {
        CacheType::Redis => {
            let mgr = crate::redis::new_redis_pool(redis_dsn()).await;
            cache::redis::new(mgr)
        }
        CacheType::RedisCluster => {
            let mgr = crate::redis::new_redis_pool_clustered(redis_dsn()).await;
            cache::redis::new(mgr)
        }
        CacheType::Memory => cache::memory::new(),
        CacheType::None => cache::none::new(),
    };

    tracing::debug!("Queue type: {:?}", cfg.queue_type);
    let (queue_tx, queue_rx) = {
        match cfg.queue_type {
            QueueType::Redis => {
                let pool = crate::redis::new_redis_pool(redis_dsn()).await;
                queue::redis::new_pair(pool, Duration::from_secs(45), queue::redis::MAIN).await
            }
            QueueType::RedisCluster => {
                let pool = crate::redis::new_redis_pool_clustered(redis_dsn()).await;
                queue::redis::new_pair(pool, Duration::from_secs(45), queue::redis::MAIN).await
            }
            QueueType::Memory => queue::memory::new_pair().await,
        }
    };

    // build our application with a route
    let app = Router::new()
        .nest("/api/v1", v1::router())
        .layer(
            ServiceBuilder::new().layer_fn(|service| IdempotencyService {
                cache: cache.clone(),
                service,
            }),
        )
        .layer(TraceLayer::new_for_http().on_request(()))
        .layer(Extension(pool.clone()))
        .layer(Extension(queue_tx.clone()))
        .layer(Extension(cfg.clone()))
        .layer(Extension(cache.clone()));

    let with_api = cfg.api_enabled;
    let with_worker = cfg.worker_enabled;

    let listen_address =
        SocketAddr::from_str(&cfg.listen_address).expect("Error parsing server listen address");
    let (server, worker_loop, expired_message_cleaner_loop) = tokio::join!(
        async {
            if with_api {
                if let Some(l) = listener {
                    tracing::debug!("API: Listening on {}", l.local_addr().unwrap());
                    axum::Server::from_tcp(l)
                        .expect("Error starting http server")
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
                worker_loop(&cfg, &pool, cache, queue_tx, queue_rx).await
            } else {
                tracing::debug!("Worker: off");
                Ok(())
            }
        },
        async {
            if with_worker {
                tracing::debug!("Expired message cleaner: Initializing");
                expired_message_cleaner_loop(&pool).await
            } else {
                tracing::debug!("Expired message cleaner: off");
                Ok(())
            }
        }
    );
    server.expect("Error initializing server");
    worker_loop.expect("Error initializing worker");
    expired_message_cleaner_loop.expect("Error initializing expired message cleaner")
}
