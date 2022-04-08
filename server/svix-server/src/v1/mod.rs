// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use axum::Router;

use crate::core::cache::RedisCache;

pub mod endpoints;
pub mod utils;

pub fn router(redis: Option<RedisCache>) -> Router {
    let ret = Router::new()
        .merge(endpoints::health::router())
        .merge(auth::router())
        .merge(endpoints::application::router(redis.clone()))
        .merge(endpoints::endpoint::router(redis.clone()))
        .merge(endpoints::event_type::router(redis.clone()))
        .merge(endpoints::message::router(redis.clone()))
        .merge(endpoints::attempt::router(redis));

    #[cfg(debug_assertions)]
    if cfg!(debug_assertions) {
        return ret.merge(development::router());
    }
    ret
}

mod auth {
    use axum::{routing::post, Router};

    use super::utils::api_not_implemented;

    pub fn router() -> Router {
        Router::new()
            .route("/auth/dashboard-access/:app_id/", post(api_not_implemented))
            .route("/auth/logout/", post(api_not_implemented))
    }
}

#[cfg(debug_assertions)]
mod development {
    use axum::{
        async_trait,
        extract::{FromRequest, RequestParts},
        routing::get,
        Json, Router,
    };

    use crate::error::{Error, Result};
    use crate::v1::utils::EmptyResponse;

    struct EchoData {
        pub headers: String,
    }

    #[async_trait]
    impl<B> FromRequest<B> for EchoData
    where
        B: Send,
    {
        type Rejection = Error;

        async fn from_request(req: &mut RequestParts<B>) -> Result<Self> {
            let headers = format!("{:?}", req.headers());
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
