// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Defines idempotency middleware for the Axum server which first looks up the given key for an
//! existing response before routing to the given endpoint's function, and caches any such results
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
use blake2::{Blake2b512, Digest};
use serde::{Deserialize, Serialize};
use tower::Service;

use super::cache::{CacheKey, CacheValue, RedisCache};

/// Returns the default exipry period for cached responses
const fn expiry_default() -> Duration {
    Duration::from_secs(60 * 60 * 12)
}

/// Returns the default expiry period for the starting lock
const fn expiry_starting() -> Duration {
    Duration::from_secs(20)
}

/// The data structure containing all necessary components of a response ready to be (de)serialized
/// from/into the cache
#[derive(Deserialize, Serialize)]
enum SerializedResponse {
    Start,
    Finished {
        code: u16,
        headers: Option<HashMap<String, String>>,
        body: Option<serde_json::Value>,
    },
}
impl CacheValue for SerializedResponse {
    type Key = IdempotencyKey;
}

struct IdempotencyKey(String);
impl IdempotencyKey {
    fn new(auth_token: &str, key: &str, url: &str) -> IdempotencyKey {
        let mut hasher = Blake2b512::new();
        hasher.update(format!("{}:{}:{}", auth_token, key, url));
        let res = hasher.finalize();
        IdempotencyKey(base64::encode(&res))
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
#[derive(Clone)]
pub struct IdempotencyService<S: Clone> {
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
                            match resp {
                                // If the value is a starting value, the operation is still in progress
                                SerializedResponse::Start => {
                                    let mut total_delay_duration =
                                        std::time::Duration::from_millis(0);
                                    let out = loop {
                                        match redis.get::<SerializedResponse>(&key).await {
                                            Ok(Some(SerializedResponse::Start)) => {
                                                if total_delay_duration > expiry_starting() {
                                                    // TODO Better response
                                                    break Ok(StatusCode::INTERNAL_SERVER_ERROR
                                                        .into_response());
                                                }

                                                total_delay_duration +=
                                                    std::time::Duration::from_millis(200);
                                                tokio::time::sleep(
                                                    std::time::Duration::from_millis(200),
                                                )
                                                .await;
                                            }
                                            Ok(Some(SerializedResponse::Finished {
                                                code,
                                                headers,
                                                body,
                                            })) => {
                                                let mut out = Json(body).into_response();

                                                let status = out.status_mut();
                                                *status = code.try_into().unwrap();

                                                if let Some(resp_headers) = headers {
                                                    let headers = out.headers_mut();
                                                    *headers = resp_headers
                                                        .iter()
                                                        .map(|(k, v)| {
                                                            (k.parse().unwrap(), v.parse().unwrap())
                                                        })
                                                        .collect();
                                                }

                                                break Ok(out);
                                            }

                                            // Start value has expired
                                            // TODO: Better response
                                            Ok(None) => {
                                                break Ok(StatusCode::INTERNAL_SERVER_ERROR
                                                    .into_response())
                                            }

                                            Err(_) => {
                                                break Ok(StatusCode::INTERNAL_SERVER_ERROR
                                                    .into_response())
                                            }
                                        }
                                    };

                                    out
                                }
                                // If a finished value, just return that response
                                SerializedResponse::Finished {
                                    code,
                                    headers,
                                    body,
                                } => {
                                    let mut out = Json(body).into_response();

                                    let status = out.status_mut();
                                    *status = code.try_into().unwrap();

                                    if let Some(resp_headers) = headers {
                                        let headers = out.headers_mut();
                                        *headers = resp_headers
                                            .iter()
                                            .map(|(k, v)| (k.parse().unwrap(), v.parse().unwrap()))
                                            .collect();
                                    }

                                    Ok(out)
                                }
                            }
                        } else {
                            // Not in redis, so get the result and cache it

                            // But first set the start value
                            if redis
                                .set(&key, &SerializedResponse::Start, expiry_default())
                                .await
                                .is_err()
                            {
                                return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                            }

                            let (parts, mut body) = service
                                .call(Request::from_parts(parts, body))
                                .await
                                .map(IntoResponse::into_response)
                                // Infallible
                                .unwrap()
                                .into_parts();

                            // TODO: Error handling where [`Result::ok`] is used as well as the
                            // [`Result::unwrap`] in the header mapping

                            let bytes = body.data().await.and_then(Result::ok);

                            let json = bytes
                                .clone()
                                .map(|b| serde_json::from_slice::<serde_json::Value>(&b));

                            let resp = SerializedResponse::Finished {
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
                                body: json.and_then(Result::ok),
                            };

                            if redis.set(&key, &resp, expiry_default()).await.is_err() {
                                return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                            }

                            let bytes = bytes.unwrap_or_default();
                            Ok(Response::from_parts(parts, Body::from(bytes)).into_response())
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

#[cfg(test)]
mod tests {
    use std::{net::TcpListener, sync::Arc};

    use axum::{extract::Extension, routing::get, Router, Server};
    use reqwest::Client;
    use tokio::{sync::Mutex, task::JoinHandle};

    use super::IdempotencyService;
    use crate::core::{
        cache::RedisCache,
        security::{default_org_id, generate_token},
    };

    /// Starts a basic Axum server with one endpoint which counts the number of times the endpoint
    /// has been polled from. This will be nested in the [`IdempotencyService`] such that, providing
    /// a key may result in the count not increasing and a prior result being displayed.
    ///
    /// This function will return a join handle to that server, its URL and an [`Arc<Mutex<usize>>`]
    /// that points to the count of the server such that its internal state may be monitored.
    async fn start_service() -> (JoinHandle<()>, String, Arc<Mutex<usize>>) {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = bb8::Pool::builder()
            .build(
                bb8_redis::RedisConnectionManager::new(cfg.redis_dsn.as_deref().unwrap()).unwrap(),
            )
            .await
            .unwrap();

        let cache = RedisCache::new(redis_pool);

        let count = Arc::new(Mutex::new(0));

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());

        let jh = tokio::spawn({
            let count = count.clone();
            async move {
                Server::from_tcp(listener)
                    .unwrap()
                    .serve(
                        Router::new()
                            .route(
                                "/",
                                IdempotencyService {
                                    redis: cache,
                                    service: get(service_endpoint),
                                },
                            )
                            .layer(Extension(count))
                            .into_make_service(),
                    )
                    .await
                    .unwrap();
            }
        });

        (jh, endpoint, count)
    }

    /// Only to be used via [`start_service`] -- this is the actual endpoint implementation
    async fn service_endpoint(Extension(count): Extension<Arc<Mutex<usize>>>) -> String {
        let mut count = count.lock().await;
        *count += 1;

        format!("{}", count)
    }

    #[tokio::test]
    async fn test_basic_idempotency() {
        let (_jh, endpoint, count) = start_service().await;
        let client = Client::new();

        // Generate a new token so that keys are unique
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();
        let token = generate_token(&cfg.jwt_secret, default_org_id())
            .unwrap()
            .to_string();

        // Sanity check on test service
        assert_eq!(*count.lock().await, 0);
        let _ = client.get(&endpoint).send().await;
        assert_eq!(*count.lock().await, 1);

        // Idempotency key not yet used -- should increment
        let resp_1 = client
            .get(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 2);

        // Now used the count should not increment
        let resp_2 = client
            .get(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 2);

        // And the responses should be equivalent
        assert_eq!(resp_1.status(), resp_2.status());
        //assert_eq!(resp_1.headers(), resp_2.headers());
        assert_eq!(resp_1.text().await.unwrap(), resp_2.text().await.unwrap());

        // No key -- should increment
        let _ = client.get(&endpoint).send().await;
        assert_eq!(*count.lock().await, 3);

        // Same key -- should not increment
        let _ = client
            .get(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await;
        assert_eq!(*count.lock().await, 3);

        // New key -- should increment
        let resp_1 = client
            .get(&endpoint)
            .header("idempotency-key", "2")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 4);

        // Old key -- shouldn't increment
        let _ = client
            .get(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await;
        assert_eq!(*count.lock().await, 4);

        // Key 2, shouldn't increment and should equal resp_1
        let resp_2 = client
            .get(&endpoint)
            .header("idempotency-key", "2")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 4);

        assert_eq!(resp_1.status(), resp_2.status());
        assert_eq!(resp_1.headers(), resp_2.headers());
        assert_eq!(resp_1.text().await.unwrap(), resp_2.text().await.unwrap());
    }
}
