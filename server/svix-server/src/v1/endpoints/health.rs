// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use sea_orm::{query::Statement, ConnectionTrait, DatabaseBackend, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::{
    core::cache::{kv_def, Cache, CacheBehavior, CacheKey, CacheValue},
    queue::{QueueTask, TaskQueueProducer},
};

async fn heartbeat() -> StatusCode {
    StatusCode::NO_CONTENT
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HealthStatus {
    Operational,
    Error(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthReport {
    pg_status: HealthStatus,

    queue_status: HealthStatus,
    cache_status: HealthStatus,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct HealthCheckCacheValue(());
kv_def!(HealthCheckCacheKey, HealthCheckCacheValue);

async fn health(
    Extension(pg): Extension<DatabaseConnection>,
    Extension(queue_tx): Extension<TaskQueueProducer>,
    Extension(cache): Extension<Cache>,
) -> Json<HealthReport> {
    // SELECT 1 FROM any table
    let pg_status = if let Err(e) = pg
        .execute(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT 1 FROM endpoint".to_owned(),
        ))
        .await
    {
        HealthStatus::Error(e.to_string())
    } else {
        HealthStatus::Operational
    };

    // Send a [`HealthCheck`] through the queue
    let queue_status = if let Err(e) = queue_tx.send(QueueTask::HealthCheck, None).await {
        HealthStatus::Error(e.to_string())
    } else {
        HealthStatus::Operational
    };

    // Set a cache value
    let cache_status = if let Err(e) = cache
        .set(
            &HealthCheckCacheKey("health_check_value".to_owned()),
            &HealthCheckCacheValue(()),
            // Expires after this time, so it won't pollute the DB
            Duration::from_millis(100),
        )
        .await
    {
        HealthStatus::Error(e.to_string())
    } else {
        HealthStatus::Operational
    };

    Json(HealthReport {
        pg_status,
        queue_status,
        cache_status,
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/heartbeat/", get(heartbeat).head(heartbeat))
        .route("/health/", get(health).head(health))
}
