//! Defines idempotency middleware for the Axum server which first looks up the given key for an
//! existing resposne before routing to the given endpoint's function, and caches any such results
//! such that subsequent requests to that endpoint with the same key will return the same response.
//!
//! Responses are cached for twelve hours by default.

use std::{collections::HashMap, convert::Infallible, future::Future, pin::Pin, time::Duration};

use axum::{
    body::{Body, BoxBody, HttpBody},
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use tower::Service;

use super::cache::{CacheKey, CacheValue, RedisCache};

const fn expiry_default() -> Duration {
    Duration::from_secs(60 * 60 * 12)
}

/// The data structure containing all necessary components of a response ready to be (de)serialiZed
/// from/into the cache
#[derive(Deserialize, Serialize)]
struct SerializedResponse {
    code: u16,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
}
impl CacheValue for SerializedResponse {
    type Key = IdempotencyKey;
}

struct IdempotencyKey(String);
impl IdempotencyKey {
    fn new(auth_token: &str, key: &str, url: &str) -> IdempotencyKey {
        IdempotencyKey(format!("{}:{}:{}", auth_token, key, url))
    }
}
impl CacheKey for IdempotencyKey {
    const PREFIX_CACHE: &'static str = "SVIX_IDEMPOTENCY_CACHE";
}
impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// The idempotency middleware itself -- inserted in place of a regular route
pub struct IdempotencyService<S> {
    redis: RedisCache,
    service: S,
}

impl<S> Service<Request<Body>> for IdempotencyService<S>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Response: IntoResponse,
    S::Future: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut service = self.service.clone();
        let redis = self.redis.clone();

        Box::pin(async move {
            let (parts, body) = req.into_parts();

            if let (Some(auth), Some(key)) = (
                parts.headers.get("Authorization"),
                parts.headers.get("idempotency-key"),
            ) {
                // Some idempotency-key -- try to convert to &str
                if let (Ok(auth), Ok(key)) = (auth.to_str(), key.to_str()) {
                    let uri = parts.uri.to_string();
                    let key = IdempotencyKey::new(auth, key, &uri);

                    // Check Redis now
                    if let Ok(resp) = redis.get::<SerializedResponse>(&key).await {
                        if let Some(resp) = resp {
                            let mut out = Json(resp.body).into_response();

                            let status = out.status_mut();
                            *status = resp.code.try_into().unwrap();

                            if let Some(resp_headers) = resp.headers {
                                let headers = out.headers_mut();
                                *headers = resp_headers
                                    .iter()
                                    .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
                                    .collect();
                            }

                            Ok(out)
                        } else {
                            // Not in redis, so get the result and cache it
                            let (parts, mut body) = service
                                .call(Request::from_parts(parts, body))
                                .await
                                .map(IntoResponse::into_response)
                                // Infallible
                                .unwrap()
                                .into_parts();

                            // TODO: Error handling where [`Result::ok`] is used as well as the
                            // [`Result::unwrap`] in the header mapping

                            let bytes = body
                                .data()
                                .await
                                .map(|result| result.ok())
                                .flatten()
                                .map(|b| serde_json::from_slice::<serde_json::Value>(&b));

                            let resp = SerializedResponse {
                                code: parts.status.into(),
                                headers: Some(
                                    parts
                                        .headers
                                        .iter()
                                        .map(|(k, v)| {
                                            (k.as_str().to_owned(), v.to_str().unwrap().to_owned())
                                        })
                                        .collect(),
                                ),
                                body: bytes.map(Result::ok).flatten(),
                            };

                            if redis.set(&key, &resp, expiry_default()).await.is_err() {
                                return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                            }

                            Ok(Response::from_parts(parts, body))
                        }
                    } else {
                        // If redis errors, return an error
                        Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                    }
                } else {
                    // If conversion to &str fails, return err
                    Ok(StatusCode::UNPROCESSABLE_ENTITY.into_response())
                }
            } else {
                // No idempotency-key -- pass off to service and do not cache
                service
                    .call(Request::from_parts(parts, body))
                    .await
                    .map(IntoResponse::into_response)
            }
        })
    }
}
