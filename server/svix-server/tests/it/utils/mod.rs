// Dead code is allowed because not everything is used in all of the tests
#![allow(dead_code)]

use std::{
    future::Future,
    net::TcpListener,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, Result};
use axum::response::IntoResponse;
use http::HeaderMap;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use svix_ksuid::KsuidLike;
use svix_server::{
    cfg::ConfigurationInner,
    core::{
        security::generate_org_token,
        types::{BaseId, OrganizationId},
    },
    setup_tracing,
};
use tokio::sync::mpsc;
use tracing::instrument::WithSubscriber;

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

impl TestClient {
    pub fn new(base_uri: String, auth_token: &str) -> TestClient {
        TestClient {
            base_uri,
            auth_header: format!("Bearer {auth_token}"),
            client: Client::new(),
        }
    }

    fn build_uri(&self, endpoint: &str) -> String {
        format!("{}/{endpoint}", self.base_uri)
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
                "assertion failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        resp.json()
            .await
            .context("error receiving/parsing response")
    }

    pub async fn get_without_response(
        &self,
        endpoint: &str,
        expected_code: StatusCode,
    ) -> Result<()> {
        let mut req = self.client.get(self.build_uri(endpoint));
        req = self.add_headers(req);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertion failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        let res_body = resp.text().await.context("error receiving response")?;
        anyhow::ensure!(res_body.is_empty());

        Ok(())
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
                        "assertion failed: expected status {}, actual status {}",
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

    pub async fn post_without_response<I: Serialize>(
        &self,
        endpoint: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<()> {
        let mut req = self.client.post(self.build_uri(endpoint));
        req = self.add_headers(req).json(&input);

        let resp = req.send().await;
        match resp {
            Ok(resp) => {
                if resp.status() != expected_code {
                    anyhow::bail!(
                        "assertion failed: expected status {}, actual status {}",
                        expected_code,
                        resp.status()
                    );
                }

                Ok(())
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
                "assertion failed: expected status {}, actual status {}",
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
                "assertion failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        resp.json()
            .await
            .context("error receiving/parsing response")
    }

    pub async fn put_without_response<I: Serialize>(
        &self,
        endpoint: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<()> {
        let mut req = self.client.put(self.build_uri(endpoint));
        req = self.add_headers(req).json(&input);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertion failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        let res_body = resp.text().await.context("error receiving response")?;
        anyhow::ensure!(res_body.is_empty());

        Ok(())
    }

    pub async fn delete(&self, endpoint: &str, expected_code: StatusCode) -> Result<()> {
        let mut req = self.client.delete(self.build_uri(endpoint));
        req = self.add_headers(req);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertion failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        if expected_code == StatusCode::NO_CONTENT {
            let res_body = resp.text().await.context("error receiving response")?;
            anyhow::ensure!(res_body.is_empty());
        }

        Ok(())
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
                "assertion failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        resp.json()
            .await
            .context("error receiving/parsing response")
    }

    pub async fn patch_without_response<I: Serialize>(
        &self,
        endpoint: &str,
        input: I,
        expected_code: StatusCode,
    ) -> Result<()> {
        let mut req = self.client.patch(self.build_uri(endpoint));
        req = self.add_headers(req).json(&input);

        let resp = req.send().await.context("error sending request")?;

        if resp.status() != expected_code {
            anyhow::bail!(
                "assertion failed: expected status {}, actual status {}",
                expected_code,
                resp.status()
            );
        }

        let res_body = resp.text().await.context("error receiving response")?;
        anyhow::ensure!(res_body.is_empty());

        Ok(())
    }
}

pub fn get_default_test_config() -> ConfigurationInner {
    let _ = dotenvy::dotenv();
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
    let prefix = svix_ksuid::Ksuid::new(None, None).to_string();
    start_svix_server_with_cfg_and_org_id_and_prefix(cfg, org_id, prefix).await
}

pub async fn start_svix_server_with_cfg_and_org_id_and_prefix(
    cfg: &ConfigurationInner,
    org_id: OrganizationId,
    prefix: String,
) -> (TestClient, tokio::task::JoinHandle<()>) {
    let (tracing_subscriber, _guard) = setup_tracing(cfg, /* for_test = */ true);

    let cfg = Arc::new(cfg.clone());

    let token = generate_org_token(&cfg.jwt_signing_config, org_id).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_uri = format!("http://{}", listener.local_addr().unwrap());

    // Could update this fn to take a tokio TcpListener instead, but that's a pretty large diff
    // for very little benefit (since this is just test code anyways).
    listener.set_nonblocking(true).unwrap();
    let listener = tokio::net::TcpListener::from_std(listener).unwrap();

    let jh = tokio::spawn(
        svix_server::run_with_prefix(Some(prefix), cfg, Some(listener))
            .with_subscriber(tracing_subscriber),
    );

    (TestClient::new(base_uri, &token), jh)
}

#[derive(Debug)]
pub struct TestReceiver {
    pub endpoint: String,
    pub jh: tokio::task::JoinHandle<()>,
    pub data_recv: mpsc::Receiver<serde_json::Value>,
    pub header_recv: mpsc::Receiver<HeaderMap>,
    pub response_status_code: Arc<Mutex<ResponseStatusCode>>,
}

#[derive(Clone)]
pub struct TestAppState<T: IntoResponse + Clone> {
    tx: mpsc::Sender<serde_json::Value>,
    header_tx: mpsc::Sender<HeaderMap>,
    response_status_code: Arc<Mutex<ResponseStatusCode>>,
    response_body: T,
}

#[derive(Debug, Clone)]
pub struct ResponseStatusCode {
    pub status_code: axum::http::StatusCode,
}

impl TestReceiver {
    pub fn start(resp_with: axum::http::StatusCode) -> Self {
        Self::start_with_body(resp_with, ())
    }

    pub fn start_with_body<T>(resp_with: axum::http::StatusCode, body: T) -> Self
    where
        T: IntoResponse + Clone + Send + Sync + 'static,
    {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let listener = tokio::net::TcpListener::from_std(listener).unwrap();

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
            .with_state(TestAppState {
                tx,
                header_tx,
                response_status_code: response_status_code.clone(),
                response_body: body,
            })
            .into_make_service();

        let jh = tokio::spawn(async move {
            axum::serve(listener, routes).await.unwrap();
        });

        TestReceiver {
            endpoint,
            jh,
            data_recv,
            header_recv,
            response_status_code,
        }
    }

    pub(crate) fn try_recv_body_value(
        &mut self,
    ) -> Result<serde_json::Value, mpsc::error::TryRecvError> {
        let payload = self.data_recv.try_recv()?;
        Ok(serde_json::from_value(payload).unwrap())
    }

    pub(crate) async fn recv_body(&mut self) -> Option<serde_json::Value> {
        self.data_recv.recv().await
    }

    pub(crate) async fn recv_body_value(&mut self) -> Option<serde_json::Value> {
        let payload = tokio::time::timeout(Duration::from_secs(30), self.data_recv.recv())
            .await
            .expect("timed out")?;
        Some(serde_json::from_value(payload).unwrap())
    }

    pub fn set_response_status_code(&self, resp_with: axum::http::StatusCode) {
        self.response_status_code.lock().unwrap().status_code = resp_with;
    }
}

async fn test_receiver_route<T: IntoResponse + Clone>(
    axum::extract::State(TestAppState {
        tx,
        header_tx,
        response_status_code,
        response_body,
    }): axum::extract::State<TestAppState<T>>,
    headers: HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> (axum::http::StatusCode, impl IntoResponse) {
    tx.send(json).await.unwrap();
    header_tx.send(headers).await.unwrap();
    (
        response_status_code.lock().unwrap().status_code,
        response_body,
    )
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
