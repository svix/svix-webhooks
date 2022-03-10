// Everything is unused because only `cargo test` is used with this crate
#![allow(dead_code)]

/// A crate meant for end-to-end integration testing of the Svix server. These tests assume an
/// already running Svix server and authentication token which are given to the tests via the
/// environmental variables `SVIX_SERVER_URI` and `SVIX_SERVER_TOKEN`.
mod request_util;
#[cfg(test)]
mod tests;

use std::{
    env,
    net::{SocketAddr, TcpListener},
};

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    headers::HeaderMap,
    routing::post,
    Json, Router,
};
use tokio::sync::{mpsc, oneshot};

pub struct SvixUri(pub String);
pub struct SvixToken(pub String);

/// Fetches the environmental variables needed to connect to and authenticate with a running Svix
/// server
fn fetch_env_vars() -> (SvixUri, SvixToken) {
    let _ = dotenv::dotenv();

    (
        SvixUri(env::var("SVIX_SERVER_URL").unwrap()),
        SvixToken(env::var("SVIX_AUTH_TOKEN").unwrap()),
    )
}

/// Sets up an [`axum`] web server which lisetens on the root route for requests and performs the
/// given assertation on the headers and payload of the request. This is used for testing the proper
/// dispatch of webhooks.
///
/// NOTE: The server handles one request before shutting down
///
/// The `uri_tx` passed to the function will transmit the IP, port, and route that the webhook event
/// should be sent to.
async fn echo_web_server<F: Fn(HeaderMap, String) -> bool + Send + Sync + Clone>(
    assertation: F,
    uri_tx: oneshot::Sender<String>,
) -> bool {
    let (tx_h, mut rx_h) = mpsc::channel::<HeaderMap>(1);
    let (tx_b, mut rx_b) = mpsc::channel::<String>(1);

    let app = Router::new().route(
        "/",
        post(|headers: Headers, body: String| async move {
            tx_h.send(headers.0).await.unwrap();
            tx_b.send(body).await.unwrap();

            Json(serde_json::Value::Null)
        }),
    );
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();

    uri_tx
        .send(format!("http://{}", listener.local_addr().unwrap()))
        .unwrap();

    let handle = tokio::spawn(
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service()),
    );

    let (headers, body) = tokio::join!(rx_h.recv(), rx_b.recv());
    handle.abort();

    assertation(headers.unwrap(), body.unwrap())
}

pub struct Headers(pub HeaderMap);

#[async_trait]
impl<B> FromRequest<B> for Headers
where
    B: Send,
{
    type Rejection = ();

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, ()> {
        Ok(Headers(req.headers().unwrap().clone()))
    }
}
