use std::{net::TcpListener, sync::Arc};

use axum::{body::Body, extract::State};
use http::{HeaderValue, Request, StatusCode, Version, header::USER_AGENT};
use http_body_util::BodyExt as _;
use serde::{Deserialize, Serialize};
use svix_server::core::webhook_http_client::{Error, RequestBuilder, WebhookClient};
use tokio::sync::mpsc;

pub struct TestReceiver {
    pub uri: String,
    pub server_jh: tokio::task::JoinHandle<()>,
    pub req_recv: mpsc::Receiver<Request<Body>>,
}

impl Drop for TestReceiver {
    fn drop(&mut self) {
        self.server_jh.abort();
    }
}

#[derive(Clone)]
struct TestAppState {
    tx: mpsc::Sender<Request<Body>>,
    response_status_code: StatusCode,
}

impl TestReceiver {
    pub fn start(resp_code: StatusCode) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let listener = tokio::net::TcpListener::from_std(listener).unwrap();

        let uri = format!("http://{}/", listener.local_addr().unwrap());

        let (tx, req_recv) = mpsc::channel(32);

        let routes = axum::Router::new()
            .route("/", axum::routing::any(test_receiver_route))
            .with_state(TestAppState {
                tx,
                response_status_code: resp_code,
            })
            .into_make_service();

        let server_jh = tokio::spawn(async move {
            axum::serve(listener, routes).await.unwrap();
        });

        TestReceiver {
            uri,
            server_jh,
            req_recv,
        }
    }
}

async fn test_receiver_route(
    State(TestAppState {
        ref tx,
        response_status_code,
    }): State<TestAppState>,
    req: Request<Body>,
) -> axum::http::StatusCode {
    tx.send(req).await.unwrap();
    response_status_code
}

#[derive(Deserialize, Serialize)]
pub struct TestSerializable {
    test: String,
}

#[ignore]
#[tokio::test]
async fn test_client_basic_operation() {
    // Compares output to `reqwest`.

    let our_client = WebhookClient::new(
        Some(Arc::new(vec!["127.0.0.1/0".parse().unwrap()])),
        None,
        false,
        None,
    );
    let reqwest_client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Invalid reqwest Client configuration");

    let mut receiver = TestReceiver::start(StatusCode::OK);

    let our_req = RequestBuilder::new()
        .uri_str(&receiver.uri)
        .unwrap()
        .json_body(TestSerializable {
            test: "value".to_owned(),
        })
        .unwrap()
        .version(Version::HTTP_11)
        .build()
        .unwrap();

    let _resp = our_client.execute(our_req).await.unwrap();

    let our_http_req = receiver.req_recv.recv().await.unwrap();

    let _resp = reqwest_client
        .post(&receiver.uri)
        .header(
            USER_AGENT,
            HeaderValue::from_static(concat!("Svix-Webhooks/", env!("CARGO_PKG_VERSION"))),
        )
        .version(Version::HTTP_11)
        .json(&TestSerializable {
            test: "value".to_owned(),
        })
        .send()
        .await
        .unwrap();

    let reqwest_http_req = receiver.req_recv.recv().await.unwrap();

    assert_eq!(our_http_req.headers(), reqwest_http_req.headers());

    let our_body = our_http_req.into_body().collect().await.unwrap().to_bytes();
    #[rustfmt::skip]
    let reqwest_body = reqwest_http_req.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(our_body, reqwest_body);
}

#[tokio::test]
async fn test_filtering() {
    let our_client = WebhookClient::new(None, None, false, None);

    let our_req = RequestBuilder::new()
        .uri_str("http://127.0.0.1/")
        .unwrap()
        .json_body(TestSerializable {
            test: "value".to_owned(),
        })
        .unwrap()
        .version(Version::HTTP_11)
        .build()
        .unwrap();

    assert!(matches!(
        our_client.execute(our_req).await.unwrap_err(),
        Error::BlockedIp
    ));

    let our_req = RequestBuilder::new()
        .uri_str("http://localhost/")
        .unwrap()
        .json_body(TestSerializable {
            test: "value".to_owned(),
        })
        .unwrap()
        .version(Version::HTTP_11)
        .build()
        .unwrap();

    assert!(matches!(
        our_client.execute(our_req).await.unwrap_err(),
        Error::BlockedIp
    ));
}
