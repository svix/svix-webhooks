// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

#![warn(clippy::all)]
#![forbid(unsafe_code)]

use axum::{extract::Extension, Router};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
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

    let (redis_pool, redis_cache): (
        Option<Pool<RedisConnectionManager>>,
        Option<RedisCache<RedisConnectionManager>>,
    ) = if let QueueType::Redis = cfg.queue_type {
        let mgr = RedisConnectionManager::new(
            cfg.redis_dsn.as_ref().unwrap().get(0).unwrap().to_string(),
        )
        .unwrap();
        let pool = bb8::Pool::builder().build(mgr).await.unwrap();
        let cache = RedisCache::new(pool.clone());
        (Some(pool), Some(cache))
    } else {
        (None, None)
    };

    let (redis_cluster_pool, redis_cluster_cache): (
        Option<Pool<RedisClusterConnectionManager>>,
        Option<RedisCache<RedisClusterConnectionManager>>,
    ) = if let QueueType::RedisCluster = cfg.queue_type {
        let mgr =
            RedisClusterConnectionManager::new(cfg.redis_dsn.as_ref().unwrap().clone()).unwrap();
        let pool = bb8::Pool::builder().build(mgr).await.unwrap();
        let cache = RedisCache::new(pool.clone());
        (Some(pool), Some(cache))
    } else {
        (None, None)
    };

    tracing::debug!("Queue type: {:?}", cfg.queue_type);
    let (queue_tx, queue_rx) = match cfg.queue_type {
        QueueType::Memory => queue::memory::new_pair().await,
        QueueType::Redis => {
            queue::redis::new_pair(
                redis_pool
                    .as_ref()
                    .expect("Failed to initialize queue: no pool available")
                    .clone(),
            )
            .await
        }
        QueueType::RedisCluster => {
            queue::redis::new_pair(
                redis_cluster_pool
                    .as_ref()
                    .expect("Failed to initialize queue: No clustered pool available")
                    .clone(),
            )
            .await
        }
    };

    // build our application with a route
    let mut app = Router::new();

    if let QueueType::RedisCluster = cfg.queue_type {
        app = app.nest("/api/v1", v1::router::<RedisClusterConnectionManager>());
    } else {
        app = app.nest("/api/v1", v1::router::<RedisConnectionManager>());
    }

    let app = app
        .layer(TraceLayer::new_for_http().on_request(()))
        .layer(Extension(pool.clone()))
        .layer(Extension(queue_tx.clone()))
        .layer(Extension(cfg.clone()))
        .layer(Extension(redis_cache.clone()))
        .layer(Extension(redis_cluster_cache.clone()));

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
