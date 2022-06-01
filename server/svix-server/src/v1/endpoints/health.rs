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

async fn ping() -> StatusCode {
    StatusCode::NO_CONTENT
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HealthStatusVariant {
    Ok,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    status: HealthStatusVariant,
    information: String,
}

impl HealthStatus {
    pub fn new_ok() -> HealthStatus {
        HealthStatus {
            status: HealthStatusVariant::Ok,
            information: "OK".to_owned(),
        }
    }

    pub fn new_error(information: String) -> HealthStatus {
        HealthStatus {
            status: HealthStatusVariant::Error,
            information,
        }
    }
}
impl<O, E: std::error::Error> From<Result<O, E>> for HealthStatus {
    fn from(res: Result<O, E>) -> Self {
        match res {
            Ok(_) => HealthStatus::new_ok(),
            Err(e) => HealthStatus::new_error(e.to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthReport {
    database: HealthStatus,

    queue: HealthStatus,
    cache: HealthStatus,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct HealthCheckCacheValue(());
kv_def!(HealthCheckCacheKey, HealthCheckCacheValue);

async fn health(
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(queue_tx): Extension<TaskQueueProducer>,
    Extension(cache): Extension<Cache>,
) -> Json<HealthReport> {
    // SELECT 1 FROM any table
    let database = db
        .execute(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT 1".to_owned(),
        ))
        .await
        .into();

    // Send a [`HealthCheck`] through the queue
    let queue = queue_tx.send(QueueTask::HealthCheck, None).await.into();

    // Set a cache value with an expiration to ensure it works
    let cache = cache
        .set(
            &HealthCheckCacheKey("health_check_value".to_owned()),
            &HealthCheckCacheValue(()),
            // Expires after this time, so it won't pollute the DB
            Duration::from_millis(100),
        )
        .await
        .into();

    Json(HealthReport {
        database,
        queue,
        cache,
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/ping/", get(ping).head(ping))
        .route("/health/", get(health).head(health))
}
