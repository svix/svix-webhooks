// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use axum::{http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};

async fn health() -> (StatusCode, Json<Value>) {
    (StatusCode::NO_CONTENT, Json(json!({})))
}

pub fn router() -> Router {
    Router::new().route("/health/", get(health).head(health))
}
