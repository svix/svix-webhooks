// Dead code is allowed because not everything is used in all of the tests
#![allow(dead_code)]

use std::{
    future::Future,
    net::TcpListener,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::mpsc;

use svix_ksuid::KsuidLike;
use svix_server::{
    cfg::ConfigurationInner,
    core::{
        security::generate_org_token,
        types::{BaseId, OrganizationId},
    },
    setup_tracing,
};

use http::HeaderMap;

pub mod common_calls;

#[derive(Clone)]
pub struct TestClient {
    base_uri: String,
    auth_header: String,
    client: Client,
}

impl TestClient {
    pub fn set_auth_header(&mut self, auth_header: String) {
        self.auth_header = format!("Bearer {auth_header}");
    }
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
            auth_header: format!("Bearer {auth_token}"),
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

        resp.json()
            .await
            .context("error receiving/parsing response")
    }

    pub async fn post<I: Serialize, O: DeserializeOwned>(
        &self,
        endpoint: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<O> {
        let mut req = self.client.post(self.build_uri(endpoint));
        req = self.add_headers(req).json(&input);

        let resp = req.send().await;
        match resp {
            Ok(resp) => {
                if resp.status() != expected_code {
                    anyhow::bail!(
                        "assertation failed: expected status {}, actual status {}",
                        expected_code,
                        resp.status()
                    );
                }

                resp.json()
                    .await
                    .context("error receiving/parsing response")
            }
            Err(e) => {
                println!("Unexpected request error: {e:?}");
                Err(e.into())
            }
        }
    }

    pub async fn post_with_idempotency<I: Serialize, O: DeserializeOwned>(
        &self,
        endpoint: &str,
        idempotency_key: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<O> {
        let mut req = self.client.post(self.build_uri(endpoint));
        req = self
            .add_headers(req)
            .header("idempotency-key", idempotency_key)
            .json(&input);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertation failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        resp.json().await.context("error receiving/paring response")
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

        resp.json()
            .await
            .context("error receiving/parsing response")
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

        resp.json()
            .await
            .context("error receiving/parsing response")
    }

    pub async fn patch<I: Serialize, O: DeserializeOwned>(
        &self,
        endpoint: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<O> {
        let mut req = self.client.patch(self.build_uri(endpoint));
        req = self.add_headers(req).json(&input);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertation failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        resp.json()
            .await
            .context("error receiving/parsing response")
    }
}

pub fn get_default_test_config() -> ConfigurationInner {
    let _ = dotenv::dotenv();
    let cfg = svix_server::cfg::load().unwrap();

    cfg.as_ref().clone()
}

pub async fn start_svix_server() -> (TestClient, tokio::task::JoinHandle<()>) {
    start_svix_server_with_cfg(&get_default_test_config()).await
}

pub async fn start_svix_server_with_cfg(
    cfg: &ConfigurationInner,
) -> (TestClient, tokio::task::JoinHandle<()>) {
    start_svix_server_with_cfg_and_org_id(cfg, OrganizationId::new(None, None)).await
}

pub async fn start_svix_server_with_cfg_and_org_id(
    cfg: &ConfigurationInner,
    org_id: OrganizationId,
) -> (TestClient, tokio::task::JoinHandle<()>) {
    setup_tracing(cfg);

    let cfg = Arc::new(cfg.clone());

    let token = generate_org_token(&cfg.jwt_secret, org_id).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_uri = format!("http://{}", listener.local_addr().unwrap());

    let jh = tokio::spawn(svix_server::run_with_prefix(
        Some(svix_ksuid::Ksuid::new(None, None).to_string()),
        cfg,
        Some(listener),
    ));

    (TestClient::new(base_uri, &token), jh)
}

pub struct TestReceiver {
    pub endpoint: String,
    pub jh: tokio::task::JoinHandle<()>,
    pub data_recv: mpsc::Receiver<serde_json::Value>,
    pub header_recv: mpsc::Receiver<HeaderMap>,
    pub response_status_code: Arc<Mutex<ResponseStatusCode>>,
}

#[derive(Clone)]
pub struct ResponseStatusCode {
    pub status_code: axum::http::StatusCode,
}

impl TestReceiver {
    pub fn start(resp_with: axum::http::StatusCode) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());

        let (tx, data_recv) = mpsc::channel(32);
        let (header_tx, header_recv) = mpsc::channel(32);

        let response_status_code = Arc::new(Mutex::new(ResponseStatusCode {
            status_code: resp_with,
        }));

        let routes = axum::Router::new()
            .route(
                "/",
                axum::routing::post(test_receiver_route).get(test_receiver_route),
            )
            .layer(axum::extract::Extension(tx))
            .layer(axum::extract::Extension(header_tx))
            .layer(axum::extract::Extension(response_status_code.clone()))
            .into_make_service();

        let jh = tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(routes)
                .await
                .unwrap();
        });

        TestReceiver {
            endpoint,
            jh,
            data_recv,
            header_recv,
            response_status_code,
        }
    }

    pub fn set_response_status_code(&self, resp_with: axum::http::StatusCode) {
        self.response_status_code.lock().unwrap().status_code = resp_with;
    }
}

async fn test_receiver_route(
    axum::extract::Extension(ref tx): axum::extract::Extension<mpsc::Sender<serde_json::Value>>,
    axum::extract::Extension(ref header_tx): axum::extract::Extension<mpsc::Sender<HeaderMap>>,
    axum::extract::Extension(response_status_code): axum::extract::Extension<
        Arc<Mutex<ResponseStatusCode>>,
    >,
    headers: HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> axum::http::StatusCode {
    tx.send(json).await.unwrap();
    header_tx.send(headers).await.unwrap();
    response_status_code.lock().unwrap().status_code
}

pub async fn run_with_retries<O, F, C>(f: C) -> Result<O>
where
    F: Future<Output = Result<O>>,
    C: Fn() -> F,
{
    for attempt in 0..50 {
        let out = f().await;
        if out.is_ok() {
            return out;
        } else if let Err(err) = out {
            println!("Attempt {attempt}: {err}");
        }

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    anyhow::bail!("All attempts failed");
}
