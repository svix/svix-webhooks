// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use axum::{http::StatusCode, routing::get, Router};

async fn health() -> StatusCode {
    StatusCode::NO_CONTENT
}

pub fn router() -> Router {
    Router::new().route("/health/", get(health).head(health))
}
