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
use http::request::Parts;

use serde::{Deserialize, Serialize};
use tower::Service;

use super::cache::{kv_def, Cache, CacheBehavior, CacheKey, CacheValue};
use crate::{err_database, error::Error};

/// Returns the default exipry period for cached responses
const fn expiry_default() -> Duration {
    Duration::from_secs(60 * 60 * 12)
}

/// Returns the default expiry period for the starting lock
const fn expiry_starting() -> Duration {
    Duration::from_secs(5)
}

/// Returns the duration to sleep before retrying to find a [`SerializedResponse::Finished`] in the
/// cache
const fn wait_duration() -> Duration {
    Duration::from_millis(200)
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

kv_def!(IdempotencyKey, SerializedResponse, "SVIX_IDEMPOTENCY_CACHE");

impl IdempotencyKey {
    fn new(auth_token: &str, key: &str, url: &str) -> IdempotencyKey {
        let mut hasher = Blake2b512::new();

        hasher.update(auth_token);
        hasher.update(":");
        hasher.update(key);
        hasher.update(":");
        hasher.update(url);

        let res = hasher.finalize();
        IdempotencyKey(base64::encode(res))
    }
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

/// The idempotency middleware itself -- used via the [`Router::layer`] method
#[derive(Clone)]
pub struct IdempotencyService<S: Clone> {
    pub cache: Cache,
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
        let cache = self.cache.clone();

        if !cache.is_none() {
            Box::pin(async move {
                let (parts, body) = req.into_parts();

                // If not a POST request, simply resolve the service as usual
                if parts.method != http::Method::POST {
                    return resolve_service(service, Request::from_parts(parts, body)).await;
                }

                // Retreive `IdempotencyKey` from header and URL parts, but returning the service
                // normally in the event a key could not be created.
                let key = if let Some(key) = get_key(&parts) {
                    key
                } else {
                    return resolve_service(service, Request::from_parts(parts, body)).await;
                };

                // Set the [`SerializedResponse::Start`] lock if the key does not exist in the cache
                // returning whether the value was set
                let lock_acquired = if let Ok(lock_acquired) = cache
                    .set_if_not_exists(&key, &SerializedResponse::Start, expiry_starting())
                    .await
                {
                    lock_acquired
                } else {
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                };

                // If the lock was not set, first check the cache for a `Finished` cache value. If
                // it is instead `None` or the value is a `Start` lock, then enter a loop checking
                // it every 200ms.
                //
                // If the loop times out, then reset the lock and proceed to resolve the service.
                //
                // If at any point the cache returns an `Err`, then return 500 response
                if !lock_acquired {
                    match cache.get::<SerializedResponse>(&key).await {
                        Ok(Some(SerializedResponse::Finished {
                            code,
                            headers,
                            body,
                        })) => {
                            return Ok(finished_serialized_response_to_reponse(code, headers, body)
                                .unwrap_or_else(|_| {
                                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                                }))
                        }

                        Ok(Some(SerializedResponse::Start)) | Ok(None) => {
                            if let Ok(Some(SerializedResponse::Finished {
                                code,
                                headers,
                                body,
                            })) = lock_loop(&cache, &key).await
                            {
                                return Ok(finished_serialized_response_to_reponse(
                                    code, headers, body,
                                )
                                .unwrap_or_else(|_| {
                                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                                }));
                            } else {
                                // Set the lock if it returns `Ok(None)` and continue to resolve
                                // as normal, but return 500 if the lock cannot be set
                                if !matches!(
                                    cache
                                        .set_if_not_exists(
                                            &key,
                                            &SerializedResponse::Start,
                                            expiry_starting(),
                                        )
                                        .await,
                                    Ok(true)
                                ) {
                                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                                }
                            }
                        }

                        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
                    }
                };

                // If it's set or the lock or the `lock_loop` returns Ok(None), then the key has no
                // value, so continue resolving the service while caching the response for 2xx
                // responses
                resolve_and_cache_response(&cache, &key, service, Request::from_parts(parts, body))
                    .await
            })
        } else {
            Box::pin(async move { Ok(service.call(req).await.into_response()) })
        }
    }
}

/// Retreives an [`IdempotencyKey`] from the [`Parts`] of a [`Request`] returning None in the event
/// that not all erquisite parts are there.
fn get_key(parts: &Parts) -> Option<IdempotencyKey> {
    let key = if let Some(Ok(key)) = parts.headers.get("idempotency-key").map(|v| v.to_str()) {
        key
    } else {
        // No idempotency-key -- pass off to service and do not cache
        return None;
    };

    let auth = if let Some(Ok(auth)) = parts.headers.get("Authorization").map(|v| v.to_str()) {
        auth
    } else {
        // No auth token -- pass off to service and do not cache
        return None;
    };

    let uri = parts.uri.to_string();

    Some(IdempotencyKey::new(auth, key, &uri))
}

/// If the lock could not be set, then another request with that key has been completed or is being
/// completed, so loop until it has been completed or times out
async fn lock_loop(
    cache: &Cache,
    key: &IdempotencyKey,
) -> Result<Option<SerializedResponse>, Error> {
    let mut total_delay_duration = std::time::Duration::from_millis(0);

    loop {
        total_delay_duration += wait_duration();
        tokio::time::sleep(wait_duration()).await;

        match cache.get::<SerializedResponse>(key).await {
            // Value has been retreived from cache, so return it
            Ok(Some(resp @ SerializedResponse::Finished { .. })) => return Ok(Some(resp)),

            // Request setting the lock has not been resolved yet, so wait a little and loop again
            Ok(Some(SerializedResponse::Start)) => {
                if total_delay_duration > expiry_starting() {
                    return Ok(None);
                }
            }

            // Start value has expired
            Ok(None) => return Ok(None),

            Err(e) => return Err(err_database!(e)),
        }
    }
}

/// Resolves the service and chaches the result assuming the response is successful
async fn resolve_and_cache_response<S>(
    cache: &Cache,
    key: &IdempotencyKey,
    service: S,
    request: Request<Body>,
) -> Result<Response<BoxBody>, Infallible>
where
    S: Service<Request<Body>, Error = Infallible> + Clone + Send + 'static,
    S::Response: IntoResponse,
    S::Future: Send + 'static,
{
    let (parts, mut body) = resolve_service(service, request)
        .await
        // Infallible
        .unwrap()
        .into_parts();

    // If a 2xx response, cache the actual response
    if parts.status.is_success() {
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

        if cache.set(key, &resp, expiry_default()).await.is_err() {
            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }

        // Assumes None to be an empty byte array
        let bytes = bytes.unwrap_or_default();
        Ok(Response::from_parts(parts, Body::from(bytes)).into_response())
    }
    // If any other status, unset the start lock and return the response
    else {
        if cache.delete(key).await.is_err() {
            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }

        Ok(Response::from_parts(parts, body).into_response())
    }
}

#[cfg(test)]
mod tests {
    use std::{net::TcpListener, sync::Arc};

    use axum::{extract::State, routing::post, Router, Server};
    use http::StatusCode;
    use reqwest::Client;
    use tokio::{sync::Mutex, task::JoinHandle};
    use tower::ServiceBuilder;

    use super::IdempotencyService;
    use crate::core::{
        cache,
        security::generate_org_token,
        types::{BaseId, OrganizationId},
    };

    #[derive(Clone)]
    struct TestAppState {
        count: Arc<Mutex<u16>>,
        wait: Option<std::time::Duration>,
    }

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
    ) -> (JoinHandle<()>, String, Arc<Mutex<u16>>) {
        dotenv::dotenv().ok();

        let cache = cache::memory::new();

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
                            .layer(ServiceBuilder::new().layer_fn(move |service| {
                                IdempotencyService {
                                    cache: cache.clone(),
                                    service,
                                }
                            }))
                            .with_state(TestAppState { count, wait })
                            .into_make_service(),
                    )
                    .await
                    .unwrap();
            }
        });

        (jh, endpoint, count)
    }

    /// Only to be used via [`start_service`] -- this is the actual endpoint implementation
    async fn service_endpoint(State(TestAppState { wait, count }): State<TestAppState>) -> String {
        let mut count = count.lock().await;
        *count += 1;

        if let Some(wait) = wait {
            tokio::time::sleep(wait).await;
        }

        format!("{count}")
    }

    #[tokio::test]
    async fn test_basic_idempotency() {
        let (_jh, endpoint, count) = start_service(None).await;
        let client = Client::new();

        // Generate a new token so that keys are unique
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();
        let token = generate_org_token(&cfg.jwt_secret, OrganizationId::new(None, None))
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

        let token = generate_org_token(&cfg.jwt_secret, OrganizationId::new(None, None))
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

        let cache = cache::memory::new();

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
                            .layer(ServiceBuilder::new().layer_fn(move |service| {
                                IdempotencyService {
                                    cache: cache.clone(),
                                    service,
                                }
                            }))
                            .with_state(TestAppState { count, wait: None })
                            .into_make_service(),
                    )
                    .await
                    .unwrap();
            }
        });

        (jh, endpoint, count)
    }

    /// Only to be used via [`start_empty_service`] -- this is the actual endpoint implementation
    async fn empty_service_endpoint(
        State(TestAppState { count, .. }): State<TestAppState>,
    ) -> StatusCode {
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
        let token = generate_org_token(&cfg.jwt_secret, OrganizationId::new(None, None))
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
