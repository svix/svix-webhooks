use std::{collections::HashMap, net::TcpListener, str::FromStr};

use anyhow::{Context, Result};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};

pub struct TestClient {
    base_uri: String,
    auth_header: String,
    client: Client,
}

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl TestClient {
    pub fn new(base_uri: String, auth_token: &str) -> TestClient {
        TestClient {
            base_uri,
            auth_header: format!("Bearer {}", auth_token),
            client: Client::new(),
        }
    }

    pub async fn request<I: Serialize, O: DeserializeOwned>(
        &self,
        endpoint: &str,
        headers: HashMap<String, String>,
        method: Method,
        body: Option<I>,
    ) -> Result<(StatusCode, Result<O>)> {
        let uri = format!("{}/{}", self.base_uri, endpoint);

        let request = match &method {
            Method::Get => self.client.get(uri),
            Method::Post => self.client.post(uri),
            Method::Put => self.client.put(uri),
            Method::Delete => self.client.delete(uri),
        };

        let request = request.header("Authorization", &self.auth_header);
        let request = request.headers(
            headers
                .iter()
                .map(|(k, v)| -> Result<(HeaderName, HeaderValue)> {
                    Ok((
                        HeaderName::from_str(k).context("Invalid HeaderName")?,
                        HeaderValue::from_str(v).context("Invalid HeaderValue")?,
                    ))
                })
                .collect::<Result<HeaderMap>>()?,
        );

        let request = if let Some(body) = body {
            request.json(&body)
        } else {
            request
        };

        let response = request.send().await?;

        Ok((
            response.status(),
            response.json::<O>().await.context("Error deserializing"),
        ))
    }

    pub async fn asserting_request<
        I: Serialize,
        O: DeserializeOwned + Clone,
        F: FnOnce(O) -> Result<()>,
    >(
        &self,
        endpoint: &str,
        headers: HashMap<String, String>,
        method: Method,
        body: Option<I>,
        expected_code: StatusCode,
        response_assertation: F,
    ) -> Result<O> {
        let (status, output): (_, Result<O>) = self
            .request(endpoint, headers, method, body)
            .await
            .context("error polling endpoint")?;

        if status != expected_code {
            anyhow::bail!(
                "assertation failed: expected {}, received {}",
                expected_code,
                status
            )
        }

        let output = output.context("failed to deserialize response")?;
        response_assertation(output.clone()).context("assertation failed")?;

        Ok(output)
    }

    pub async fn asserting_request_no_response_body<I: Serialize>(
        &self,
        endpoint: &str,
        headers: HashMap<String, String>,
        method: Method,
        body: Option<I>,
        expected_code: StatusCode,
    ) -> Result<()> {
        let (status, _): (_, Result<()>) = self
            .request(endpoint, headers, method, body)
            .await
            .context("error polling endpoint")?;

        if status != expected_code {
            anyhow::bail!(
                "assertation failed: expected {}, received {}",
                expected_code,
                status
            )
        }

        Ok(())
    }

    pub async fn asserting_get<O: DeserializeOwned + Clone + PartialEq + std::fmt::Debug>(
        &self,
        endpoint: &str,
        expected_code: StatusCode,
        expected_resp: Option<O>,
    ) -> Result<()> {
        if let Some(expected_resp) = expected_resp {
            self.asserting_request::<(), O, _>(
                endpoint,
                HashMap::new(),
                Method::Get,
                None,
                expected_code,
                move |actual_resp| {
                    if actual_resp == expected_resp {
                        Ok(())
                    } else {
                        anyhow::bail!("exppected {:?}, received {:?}", expected_resp, actual_resp)
                    }
                },
            )
            .await
            .map(|_| ())
        } else {
            self.asserting_request_no_response_body::<()>(
                endpoint,
                HashMap::new(),
                Method::Get,
                None,
                expected_code,
            )
            .await
        }
    }
}

pub fn start_svix_server() -> (TestClient, tokio::task::JoinHandle<()>) {
    let _ = dotenv::dotenv();
    let cfg = crate::cfg::load().unwrap();
    let token = crate::generate_token(&cfg.jwt_secret).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_uri = format!("http://{}", listener.local_addr().unwrap());

    let jh = tokio::spawn(crate::run(cfg, Some(listener)));

    (TestClient::new(base_uri, &token), jh)
}
