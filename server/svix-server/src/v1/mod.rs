// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use axum::Router;

pub mod endpoints;
pub mod utils;

pub fn router() -> Router {
    let ret = Router::new()
        .merge(endpoints::health::router())
        .merge(endpoints::auth::router())
        .merge(endpoints::application::router())
        .merge(endpoints::endpoint::router())
        .merge(endpoints::event_type::router())
        .merge(endpoints::message::router())
        .merge(endpoints::attempt::router());

    #[cfg(debug_assertions)]
    if cfg!(debug_assertions) {
        return ret.merge(development::router());
    }
    ret
}

#[cfg(debug_assertions)]
mod development {
    use axum::{async_trait, extract::FromRequestParts, routing::get, Json, Router};
    use http::request::Parts;

    use crate::error::{Error, Result};
    use crate::v1::utils::EmptyResponse;

    struct EchoData {
        pub headers: String,
    }

    #[async_trait]
    impl<S> FromRequestParts<S> for EchoData
    where
        S: Send + Sync,
    {
        type Rejection = Error;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
            let headers = format!("{:?}", parts.headers);
            Ok(EchoData { headers })
        }
    }

    async fn echo(data: EchoData, body: String) -> Result<Json<EmptyResponse>> {
        tracing::info!(">>> Echo");
        tracing::info!("{}", data.headers);
        tracing::info!("{}", body);
        tracing::info!("<<< Echo");
        Ok(Json(EmptyResponse {}))
    }

    pub fn router() -> Router {
        Router::new().route("/development/echo/", get(echo).post(echo))
    }
}
