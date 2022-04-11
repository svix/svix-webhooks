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
        headers: Option<HashMap<String, Vec<u8>>>,
        body: Option<Vec<u8>>,
    },
}
impl CacheValue for SerializedResponse {
    type Key = IdempotencyKey;
}

#[derive(thiserror::Error, Debug)]
pub enum ConversionToResponseError {
    #[error("the status code is out of bounds")]
    StatusError(#[from] http::status::InvalidStatusCode),

    #[error("a header name is invalid")]
    FromStr(#[from] http::header::InvalidHeaderName),
    #[error("a header value is invalid")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
}

/// Will never error as long as Redis doesn't corrupt -- never use this with anything but values
/// from Redis which were put in via the idempotency service from known good requests.
fn finished_serialized_response_to_reponse(
    code: u16,
    headers: Option<HashMap<String, Vec<u8>>>,
    body: Option<Vec<u8>>,
) -> Result<Response<BoxBody>, ConversionToResponseError> {
    let mut out = body.unwrap_or_default().into_response();

    let status = out.status_mut();
    *status = code.try_into()?;

    if let Some(resp_headers) = headers {
        let headers = out.headers_mut();
        *headers = resp_headers
            .iter()
            .map(|(k, v)| Ok((k.parse()?, http::HeaderValue::from_bytes(v)?)))
            .collect::<Result<_, ConversionToResponseError>>()?;
    }

    Ok(out)
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

async fn resolve_service<S>(
    mut service: S,
    req: Request<Body>,
) -> Result<Response<BoxBody>, Infallible>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Response: IntoResponse,
    S::Future: Send + 'static,
{
    service.call(req).await.map(IntoResponse::into_response)
}

/// The idempotency middleware itself -- inserted in place of a regular route
#[derive(Clone)]
pub struct IdempotencyService<S: Clone> {
    pub redis: Option<RedisCache>,
    pub service: S,
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

        if let Some(redis) = redis {
            Box::pin(async move {
                let (parts, body) = req.into_parts();

                // If not a POST request, simply resolve the service as usual
                if parts.method != http::Method::POST {
                    return resolve_service(service, Request::from_parts(parts, body)).await;
                }

                let auth = if let Some(Ok(auth)) =
                    parts.headers.get("Authorization").map(|v| v.to_str())
                {
                    auth
                } else {
                    // No auth token -- pass off to service and do not cache
                    return resolve_service(service, Request::from_parts(parts, body)).await;
                };

                let key = if let Some(Ok(key)) =
                    parts.headers.get("idempotency-key").map(|v| v.to_str())
                {
                    key
                } else {
                    // No idempotency-key -- pass off to service and do not cache
                    return resolve_service(service, Request::from_parts(parts, body)).await;
                };

                let uri = parts.uri.to_string();
                let key = IdempotencyKey::new(auth, key, &uri);

                // Check Redis now
                match redis.get::<SerializedResponse>(&key).await {
                    // If the value is a starting value, the operation is still in progress
                    Ok(Some(SerializedResponse::Start)) => {
                        let mut total_delay_duration = std::time::Duration::from_millis(0);
                        let out = loop {
                            match redis.get::<SerializedResponse>(&key).await {
                                Ok(Some(SerializedResponse::Start)) => {
                                    if total_delay_duration > expiry_starting() {
                                        // TODO Better response
                                        break Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                                    }

                                    total_delay_duration += std::time::Duration::from_millis(200);
                                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                                }
                                Ok(Some(SerializedResponse::Finished {
                                    code,
                                    headers,
                                    body,
                                })) => {
                                    break Ok(finished_serialized_response_to_reponse(
                                        code, headers, body,
                                    )
                                    .unwrap_or_else(|_| {
                                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                                    }));
                                }

                                // Start value has expired
                                // TODO: Better response
                                Ok(None) => {
                                    break Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                                }

                                Err(_) => {
                                    break Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response())
                                }
                            }
                        };

                        out
                    }

                    // The operation is finished and cached, so just return it
                    Ok(Some(SerializedResponse::Finished {
                        code,
                        headers,
                        body,
                    })) => Ok(finished_serialized_response_to_reponse(code, headers, body)
                        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())),

                    // Not in redis, so get the result and cache it
                    Ok(None) => {
                        // First set the start value as a lock
                        if redis
                            .set(&key, &SerializedResponse::Start, expiry_default())
                            .await
                            .is_err()
                        {
                            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                        }

                        let (parts, mut body) =
                            resolve_service(service, Request::from_parts(parts, body))
                                .await
                                // Infallible
                                .unwrap()
                                .into_parts();

                        // TODO: Don't skip over Err value
                        let bytes = body.data().await.and_then(Result::ok);

                        let resp = SerializedResponse::Finished {
                            code: parts.status.into(),
                            headers: Some(
                                parts
                                    .headers
                                    .iter()
                                    .map(|(k, v)| (k.as_str().to_owned(), v.as_bytes().to_owned()))
                                    .collect(),
                            ),
                            body: bytes.clone().map(|b| b.to_vec()),
                        };

                        if redis.set(&key, &resp, expiry_default()).await.is_err() {
                            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                        }

                        // Assumes None to be an empty byte array
                        let bytes = bytes.unwrap_or_default();
                        Ok(Response::from_parts(parts, Body::from(bytes)).into_response())
                    }

                    Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
                }
            })
        } else {
            Box::pin(async move { Ok(service.call(req).await.into_response()) })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{net::TcpListener, sync::Arc};

    use axum::{extract::Extension, routing::post, Router, Server};
    use http::StatusCode;
    use reqwest::Client;
    use tokio::{sync::Mutex, task::JoinHandle};
    use tower::ServiceBuilder;

    use super::IdempotencyService;
    use crate::core::{
        cache::RedisCache,
        security::generate_token,
        types::{BaseId, OrganizationId},
    };

    /// Starts a basic Axum server with one endpoint which counts the number of times the endpoint
    /// has been polled from. This will be nested in the [`IdempotencyService`] such that, providing
    /// a key may result in the count not increasing and a prior result being displayed.
    ///
    /// This function takes a variable length of time to complete with the delay input used for
    /// testing the start lock.
    ///
    /// This function will return a join handle to that server, its URL and an [`Arc<Mutex<usize>>`]
    /// that points to the count of the server such that its internal state may be monitored.
    async fn start_service(
        wait: Option<std::time::Duration>,
    ) -> (JoinHandle<()>, String, Arc<Mutex<usize>>) {
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
                            .route("/", post(service_endpoint).get(service_endpoint))
                            .layer(
                                ServiceBuilder::new().layer_fn(|service| IdempotencyService {
                                    redis: Some(cache.clone()),
                                    service,
                                }),
                            )
                            .layer(Extension(count))
                            .layer(Extension(wait))
                            .into_make_service(),
                    )
                    .await
                    .unwrap();
            }
        });

        (jh, endpoint, count)
    }

    /// Only to be used via [`start_service`] -- this is the actual endpoint implementation
    async fn service_endpoint(
        Extension(count): Extension<Arc<Mutex<usize>>>,
        Extension(wait): Extension<Option<std::time::Duration>>,
    ) -> String {
        let mut count = count.lock().await;
        *count += 1;

        if let Some(wait) = wait {
            tokio::time::sleep(wait).await;
        }

        format!("{}", count)
    }

    #[tokio::test]
    async fn test_basic_idempotency() {
        let (_jh, endpoint, count) = start_service(None).await;
        let client = Client::new();

        // Generate a new token so that keys are unique
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();
        let token = generate_token(&cfg.jwt_secret, OrganizationId::new(None, None))
            .unwrap()
            .to_string();

        // Sanity check on test service
        assert_eq!(*count.lock().await, 0);
        let _ = client.post(&endpoint).send().await;
        assert_eq!(*count.lock().await, 1);

        // Idempotency key not yet used -- should increment
        let resp_1 = client
            .post(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 2);

        // Now used the count should not increment
        let resp_2 = client
            .post(&endpoint)
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
        let _ = client.post(&endpoint).send().await;
        assert_eq!(*count.lock().await, 3);

        // Same key -- should not increment
        let _ = client
            .post(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await;
        assert_eq!(*count.lock().await, 3);

        // New key -- should increment
        let resp_1 = client
            .post(&endpoint)
            .header("idempotency-key", "2")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 4);

        // Old key -- shouldn't increment
        let _ = client
            .post(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await;
        assert_eq!(*count.lock().await, 4);

        // Key 2, shouldn't increment and should equal resp_1
        let resp_2 = client
            .post(&endpoint)
            .header("idempotency-key", "2")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 4);

        assert_eq!(resp_1.status(), resp_2.status());
        assert_eq!(resp_1.headers(), resp_2.headers());
        assert_eq!(resp_1.text().await.unwrap(), resp_2.text().await.unwrap());

        // Key 2, but with GET should increment as it is not a POST request
        let _ = client
            .get(&endpoint)
            .header("idempotency-key", "2")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 5);
    }

    #[tokio::test]
    async fn test_lock() {
        // The sleep interval is 200ms, so have it wait 300ms causing it to sleep twice. This
        // means it should take at least 400ms for the second task to respond.
        let (_jh, endpoint, _count) =
            start_service(Some(std::time::Duration::from_millis(300))).await;
        let client = Client::new();

        // Generate a new token so that keys are unique
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let token = generate_token(&cfg.jwt_secret, OrganizationId::new(None, None))
            .unwrap()
            .to_string();

        let start = std::time::Instant::now();

        let resp_1_jh = tokio::spawn(
            client
                .post(&endpoint)
                .header("idempotency-key", "1")
                .header("Authorization", &token)
                .send(),
        );

        // It takes a couple of ms for the lock to register
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;

        let resp_2_jh = tokio::spawn(
            client
                .post(&endpoint)
                .header("idempotency-key", "1")
                .header("Authorization", &token)
                .send(),
        );

        let resp_1 = resp_1_jh.await.unwrap().unwrap();
        let resp_1_instant = std::time::Instant::now();

        let resp_2 = resp_2_jh.await.unwrap().unwrap();
        let resp_2_instant = std::time::Instant::now();

        assert!(resp_1_instant - start < std::time::Duration::from_millis(350));
        assert!(resp_2_instant - start > std::time::Duration::from_millis(400));

        // And the responses should be equivalent
        assert_eq!(resp_1.status(), resp_2.status());
        //assert_eq!(resp_1.headers(), resp_2.headers());
        assert_eq!(resp_1.text().await.unwrap(), resp_2.text().await.unwrap());
    }

    /// Starts a server just like [`start_service`] but it returns an empty body. The count is
    /// recorded in the HTTP status code.
    async fn start_empty_service() -> (JoinHandle<()>, String, Arc<Mutex<u16>>) {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = bb8::Pool::builder()
            .build(
                bb8_redis::RedisConnectionManager::new(cfg.redis_dsn.as_deref().unwrap()).unwrap(),
            )
            .await
            .unwrap();

        let cache = RedisCache::new(redis_pool);

        let count = Arc::new(Mutex::new(199));

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());

        let jh = tokio::spawn({
            let count = count.clone();
            async move {
                Server::from_tcp(listener)
                    .unwrap()
                    .serve(
                        Router::new()
                            .route("/", post(empty_service_endpoint))
                            .layer(
                                ServiceBuilder::new().layer_fn(|service| IdempotencyService {
                                    redis: Some(cache.clone()),
                                    service,
                                }),
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

    /// Only to be used via [`start_empty_service`] -- this is the actual endpoint implementation
    async fn empty_service_endpoint(Extension(count): Extension<Arc<Mutex<u16>>>) -> StatusCode {
        let mut count = count.lock().await;
        *count += 1;

        StatusCode::from_u16(*count).unwrap()
    }

    #[tokio::test]
    async fn test_empty_body() {
        let (_jh, endpoint, count) = start_empty_service().await;
        let client = Client::new();

        // Generate a new token so that keys are unique
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();
        let token = generate_token(&cfg.jwt_secret, OrganizationId::new(None, None))
            .unwrap()
            .to_string();

        // Sanity check on test service
        assert_eq!(*count.lock().await, 199);
        let _ = client.post(&endpoint).send().await.unwrap();
        assert_eq!(*count.lock().await, 200);

        // Idempotency key not yet used -- should increment
        let resp_1 = client
            .post(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 201);

        // Now used the count should not increment
        let resp_2 = client
            .post(&endpoint)
            .header("idempotency-key", "1")
            .header("Authorization", &token)
            .send()
            .await
            .unwrap();
        assert_eq!(*count.lock().await, 201);

        // And the responses should be equivalent
        assert_eq!(resp_1.status(), resp_2.status());
        //assert_eq!(resp_1.headers(), resp_2.headers());
        assert_eq!(resp_1.text().await.unwrap(), resp_2.text().await.unwrap());
    }
}
