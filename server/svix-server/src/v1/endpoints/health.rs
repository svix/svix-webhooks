// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use aide::axum::{
    routing::{get, get_with},
    ApiRouter,
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{query::Statement, ConnectionTrait, DatabaseBackend};
use serde::{Deserialize, Serialize};
use svix_server_derive::aide_annotate;

use crate::{
    core::cache::{kv_def, CacheBehavior, CacheKey, CacheValue},
    queue::QueueTask,
    v1::utils::{openapi_tag, NoContent},
    AppState,
};

async fn ping() -> NoContent {
    NoContent
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

/// Verify the API server is up and running.
#[aide_annotate(op_id = "v1.health.get")]
async fn health(
    State(AppState {
        ref db,
        queue_tx,
        cache,
        ..
    }): State<AppState>,
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

pub fn router() -> ApiRouter<AppState> {
    let tag = openapi_tag("Health");

    ApiRouter::new()
        .api_route("/health/ping", get(ping).head(ping))
        .api_route_with(
            "/health",
            get_with(health, |op| op.response::<204, ()>().with(health_operation))
                .head_with(health, health_operation),
            tag,
        )
}
