use std::net::TcpListener;

use anyhow::{Context, Result};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::core::security::test_util::generate_token_random_org;

pub struct TestClient {
    base_uri: String,
    auth_header: String,
    client: Client,
}

pub struct EmptyResponse;
impl<'de> Deserialize<'de> for EmptyResponse {
    fn deserialize<D>(_: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(EmptyResponse)
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
    println!("{}", token);
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_uri = format!("http://{}", listener.local_addr().unwrap());

    let jh = tokio::spawn(crate::run(cfg, Some(listener)));

    (TestClient::new(base_uri, &token), jh)
}
