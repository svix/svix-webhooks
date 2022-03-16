use std::{collections::HashMap, net::TcpListener};

use anyhow::{Context, Result};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::HeaderMap,
    routing::post,
    Json, Router, Server,
};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

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
    let token = generate_token_random_org(&cfg.jwt_secret).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_uri = format!("http://{}", listener.local_addr().unwrap());

    let jh = tokio::spawn(crate::run(cfg, Some(listener)));

    (TestClient::new(base_uri, &token), jh)
}

#[derive(Debug)]
pub(crate) struct Request {
    headers: HashMap<String, String>,
    data: serde_json::Value,
}

pub(crate) fn start_test_server() -> (mpsc::Receiver<Request>, tokio::task::JoinHandle<()>, String)
{
    let (tx, rx) = mpsc::channel(16);

    let router = Router::new().route(
        "/",
        post(
            |h: HeaderMap, Json(data): Json<serde_json::Value>| async move {
                let headers = h
                    .iter()
                    .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap().to_owned()))
                    .collect();
                tx.send(Request { headers, data }).await.unwrap();
                "OK"
            },
        ),
    );

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let uri = format!("http://{}", listener.local_addr().unwrap());

    let jh = tokio::spawn(async move {
        Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });

    (rx, jh, uri)
}
