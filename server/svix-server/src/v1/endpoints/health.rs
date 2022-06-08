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
#[serde(rename_all = "lowercase")]
pub enum HealthStatusVariant {
    Ok,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    status: HealthStatusVariant,
    // TODO: information field
}

impl HealthStatus {
    pub fn new_ok() -> HealthStatus {
        HealthStatus {
            status: HealthStatusVariant::Ok,
        }
    }

    pub fn new_error() -> HealthStatus {
        HealthStatus {
            status: HealthStatusVariant::Error,
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(
            self,
            HealthStatus {
                status: HealthStatusVariant::Ok,
                ..
            }
        )
    }
}
impl<O, E> From<Result<O, E>> for HealthStatus {
    fn from(res: Result<O, E>) -> Self {
        match res {
            Ok(_) => HealthStatus::new_ok(),
            Err(_) => HealthStatus::new_error(),
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
) -> (StatusCode, Json<HealthReport>) {
    // SELECT 1 FROM any table
    let database: HealthStatus = db
        .execute(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT 1".to_owned(),
        ))
        .await
        .into();

    // Send a [`HealthCheck`] through the queue
    let queue: HealthStatus = queue_tx.send(QueueTask::HealthCheck, None).await.into();

    // Set a cache value with an expiration to ensure it works
    let cache: HealthStatus = cache
        .set(
            &HealthCheckCacheKey("health_check_value".to_owned()),
            &HealthCheckCacheValue(()),
            // Expires after this time, so it won't pollute the DB
            Duration::from_millis(100),
        )
        .await
        .into();

    let status = if database.is_ok() && queue.is_ok() && cache.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    (
        status,
        Json(HealthReport {
            database,
            queue,
            cache,
        }),
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/health/ping/", get(ping).head(ping))
        .route("/health/", get(health).head(health))
}
