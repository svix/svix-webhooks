use std::{future::Future, net::TcpListener, sync::Arc};

use anyhow::{Context, Result};

use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::core::security::test_util::generate_token_random_org;

pub struct TestClient {
    base_uri: String,
    auth_header: String,
    client: Client,
}

/// This struct accepts any JSON response and just ignores it.
pub struct IgnoredResponse;
impl<'de> Deserialize<'de> for IgnoredResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = serde_json::Value::deserialize(deserializer);
        Ok(IgnoredResponse)
    }
}

impl TestClient {
    pub fn new(base_uri: String, auth_token: &str) -> TestClient {
        TestClient {
            base_uri,
            auth_header: format!("Bearer {}", auth_token),
            client: Client::new(),
        }
    }

    fn build_uri(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_uri, endpoint)
    }

    fn add_headers(&self, request: RequestBuilder) -> RequestBuilder {
        request.header("Authorization", &self.auth_header)
    }

    pub async fn get<O: DeserializeOwned>(
        &self,
        endpoint: &str,
        expected_code: StatusCode,
    ) -> Result<O> {
        let mut req = self.client.get(self.build_uri(endpoint));
        req = self.add_headers(req);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertation failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        Ok(resp
            .json()
            .await
            .context("error receiving/parsing response")?)
    }

    pub async fn post<I: Serialize, O: DeserializeOwned>(
        &self,
        endpoint: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<O> {
        let mut req = self.client.post(self.build_uri(endpoint));
        req = self.add_headers(req).json(&input);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertation failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        Ok(resp
            .json()
            .await
            .context("error receiving/parsing response")?)
    }

    pub async fn put<I: Serialize, O: DeserializeOwned>(
        &self,
        endpoint: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<O> {
        let mut req = self.client.put(self.build_uri(endpoint));
        req = self.add_headers(req).json(&input);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertation failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        Ok(resp
            .json()
            .await
            .context("error receiving/parsing response")?)
    }

    pub async fn delete<O: DeserializeOwned>(
        &self,
        endpoint: &str,
        expected_code: StatusCode,
    ) -> Result<O> {
        let mut req = self.client.delete(self.build_uri(endpoint));
        req = self.add_headers(req);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertation failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        Ok(resp
            .json()
            .await
            .context("error receiving/parsing response")?)
    }
}

pub fn start_svix_server() -> (TestClient, tokio::task::JoinHandle<()>) {
    let _ = dotenv::dotenv();
    let cfg = crate::cfg::load().unwrap();

    // Change the queue type to in-memory. This is necessary so test workers don't pick up messages
    // from other tests whose threads then abort at the end of a test before associated database
    // transactions are complete.
    let mut cfg = cfg.as_ref().clone();
    cfg.queue_type = crate::cfg::QueueType::Memory;
    let cfg = Arc::new(cfg);

    let token = generate_token_random_org(&cfg.jwt_secret).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_uri = format!("http://{}", listener.local_addr().unwrap());

    let jh = tokio::spawn(crate::run(cfg, Some(listener)));

    (TestClient::new(base_uri, &token), jh)
}

pub struct TestReceiver {
    pub endpoint: String,
    pub jh: tokio::task::JoinHandle<()>,
    pub data_recv: mpsc::Receiver<serde_json::Value>,
}

impl TestReceiver {
    pub fn start(resp_with: axum::http::StatusCode) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());

        let (tx, data_recv) = mpsc::channel(32);

        let jh = tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(
                    axum::Router::new()
                        .route(
                            "/",
                            axum::routing::post(test_receiver_route).get(test_receiver_route),
                        )
                        .layer(axum::extract::Extension(tx))
                        .layer(axum::extract::Extension(resp_with))
                        .into_make_service(),
                )
                .await
                .unwrap();
        });

        TestReceiver {
            endpoint,
            jh,
            data_recv,
        }
    }
}

async fn test_receiver_route(
    axum::Json(json): axum::Json<serde_json::Value>,
    axum::extract::Extension(ref tx): axum::extract::Extension<mpsc::Sender<serde_json::Value>>,
    axum::extract::Extension(code): axum::extract::Extension<axum::http::StatusCode>,
) -> axum::http::StatusCode {
    tx.send(json).await.unwrap();
    code
}

pub async fn run_with_retries<F: Future<Output = Result<()>>, C: Fn() -> F>(f: C) -> Result<()> {
    for attempt in 0..20 {
        let out = f().await;
        if out.is_ok() {
            return Ok(());
        }

        println!("Attempt {}: {}", attempt, out.unwrap_err());

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    anyhow::bail!("All attempts failed");
}
