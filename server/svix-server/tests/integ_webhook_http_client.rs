use std::{net::TcpListener, sync::Arc};

use axum::extract::State;
use http::{header::USER_AGENT, HeaderValue, Request, StatusCode, Version};
use hyper::Body;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use svix_server::core::webhook_http_client::{Error, RequestBuilder, WebhookClient};

pub struct TestReceiver {
    pub uri: String,
    pub jh: tokio::task::JoinHandle<()>,
    pub req_recv: mpsc::Receiver<Request<Body>>,
}

#[derive(Clone)]
struct TestAppState {
    tx: mpsc::Sender<Request<Body>>,
    response_status_code: StatusCode,
}

impl TestReceiver {
    pub fn start(resp_code: StatusCode) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let uri = format!("http://{}/", listener.local_addr().unwrap());

        let (tx, req_recv) = mpsc::channel(32);

        let routes = axum::Router::new()
            .route("/", axum::routing::any(test_receiver_route))
            .with_state(TestAppState {
                tx,
                response_status_code: resp_code,
            })
            .into_make_service();

        let jh = tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(routes)
                .await
                .unwrap();
        });

        TestReceiver { uri, jh, req_recv }
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

    let our_client = WebhookClient::new(Some(Arc::new(vec!["127.0.0.1/0".parse().unwrap()])), None);
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
    assert_eq!(
        hyper::body::to_bytes(our_http_req.into_body())
            .await
            .unwrap(),
        hyper::body::to_bytes(reqwest_http_req.into_body())
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn test_filtering() {
    let our_client = WebhookClient::new(None, None);

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
