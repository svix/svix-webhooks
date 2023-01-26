// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use aide::axum::ApiRouter;
use tower_http::trace::TraceLayer;

use crate::{
    core::otel_spans::{AxumOtelOnFailure, AxumOtelOnResponse, AxumOtelSpanCreator},
    AppState,
};

pub mod endpoints;
pub mod utils;

pub fn router() -> ApiRouter<AppState> {
    let ret: ApiRouter<AppState> = ApiRouter::new()
        .merge(endpoints::health::router())
        .merge(endpoints::auth::router())
        .merge(endpoints::application::router())
        .merge(endpoints::endpoint::router())
        .merge(endpoints::event_type::router())
        .merge(endpoints::message::router())
        .merge(endpoints::attempt::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(AxumOtelSpanCreator)
                .on_response(AxumOtelOnResponse)
                .on_failure(AxumOtelOnFailure),
        );

    #[cfg(debug_assertions)]
    if cfg!(debug_assertions) {
        let dev_router: ApiRouter<AppState> = development::router().into();
        return ret.merge(dev_router);
    }
    ret
}

#[cfg(debug_assertions)]
mod development {
    use axum::{async_trait, extract::FromRequestParts, routing::get, Json, Router};
    use http::request::Parts;

    use crate::error::{Error, Result};
    use crate::v1::utils::EmptyResponse;
    use crate::AppState;

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

    pub fn router() -> Router<AppState> {
        Router::new().route("/development/echo/", get(echo).post(echo))
    }
}
