use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use svix_bridge_types::{
    BoxError, ForwardRequest, ReceiverOutput, TransformationConfig, TransformerInput,
    TransformerInputFormat, TransformerJob, TransformerOutput, async_trait,
    svix::webhooks::Webhook,
};
use tower::{Service, ServiceExt};

use super::router;
use crate::webhook_receiver::{
    types::{IntegrationState, InternalState},
    verification::{NoVerifier, SvixVerifier},
};

struct FakeReceiverOutput {
    tx: tokio::sync::mpsc::UnboundedSender<serde_json::Value>,
}

impl FakeReceiverOutput {
    pub fn new() -> (
        Self,
        tokio::sync::mpsc::UnboundedReceiver<serde_json::Value>,
    ) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        (Self { tx }, rx)
    }
}

#[async_trait]
impl ReceiverOutput for FakeReceiverOutput {
    fn name(&self) -> &str {
        "fake output"
    }

    async fn handle(&self, request: ForwardRequest) -> Result<(), BoxError> {
        self.tx.send(request.payload)?;
        Ok(())
    }
}

#[tokio::test]
async fn test_forwarding_no_verification() {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let (a_output, mut a_rx) = FakeReceiverOutput::new();
    let state_map = [(
        "a".into(),
        IntegrationState {
            verifier: NoVerifier.into(),
            output: Arc::new(Box::new(a_output)),
            transformation: None,
        },
    )]
    .into_iter()
    .collect();
    let state = InternalState::new(state_map, tx);
    let app = router().with_state(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook/a")
                .method("POST")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(json!({ "a": true }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = a_rx.try_recv().unwrap();
    assert_eq!(json!(forwarded), json!({"a": true}));
}

/// Registers 2 receivers and sends 1 request to each.
#[tokio::test]
async fn test_forwarding_multiple_receivers() {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let (a_output, mut a_rx) = FakeReceiverOutput::new();
    let (b_output, mut b_rx) = FakeReceiverOutput::new();
    let state_map = [
        (
            "a".into(),
            IntegrationState {
                verifier: NoVerifier.into(),
                output: Arc::new(Box::new(a_output)),
                transformation: None,
            },
        ),
        (
            "b".into(),
            IntegrationState {
                verifier: NoVerifier.into(),
                output: Arc::new(Box::new(b_output)),
                transformation: None,
            },
        ),
    ]
    .into_iter()
    .collect();
    let state = InternalState::new(state_map, tx);

    let mut app = router().with_state(state);

    let request = Request::builder()
        .uri("/webhook/a")
        .method("POST")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(json!({ "a": true }).to_string()))
        .unwrap();

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = a_rx.try_recv().unwrap();
    assert_eq!(json!(forwarded), json!({"a": true}));

    let request = Request::builder()
        .uri("/webhook/b")
        .method("POST")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(json!({ "b": true }).to_string()))
        .unwrap();

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = b_rx.try_recv().unwrap();
    assert_eq!(json!(forwarded), json!({"b": true}));

    // Both channels should be empty at this point.
    assert!(a_rx.try_recv().is_err());
    assert!(b_rx.try_recv().is_err());
}

/// Registers 2 receivers, one with a transformation and one without. Sends 1 request to each.
#[tokio::test]
async fn test_transformation_json() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = rx.recv().await {
            let mut input = match x.input {
                TransformerInput::Json(input) => input.as_object().unwrap().clone(),
                _ => unreachable!(),
            };
            input.insert("__TRANSFORMED__".into(), json!(true));
            let out = json!({ "payload": input });

            x.callback_tx
                .send(Ok(TransformerOutput::Object(
                    out.as_object().unwrap().clone(),
                )))
                .ok();
        }
    });

    let (a_output, mut a_rx) = FakeReceiverOutput::new();
    let (b_output, mut b_rx) = FakeReceiverOutput::new();
    let state_map = [
        (
            "transformed".into(),
            IntegrationState {
                verifier: NoVerifier.into(),
                output: Arc::new(Box::new(a_output)),
                transformation: Some(
                    "handler = (x) => ({ payload: {__TRANSFORMED__: true, ...x }})".into(),
                ),
            },
        ),
        (
            "as-is".into(),
            IntegrationState {
                verifier: NoVerifier.into(),
                output: Arc::new(Box::new(b_output)),
                transformation: None,
            },
        ),
    ]
    .into_iter()
    .collect();
    let state = InternalState::new(state_map, tx);

    let mut app = router().with_state(state);

    let request = Request::builder()
        .uri("/webhook/transformed")
        .method("POST")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(json!({ "a": true }).to_string()))
        .unwrap();

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = a_rx.try_recv().unwrap();
    // The `__TRANSFORMED__` key should have been added
    assert_eq!(
        json!(forwarded),
        json!({"a": true, "__TRANSFORMED__": true})
    );

    let request = Request::builder()
        .uri("/webhook/as-is")
        .method("POST")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(json!({ "b": true }).to_string()))
        .unwrap();

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = b_rx.try_recv().unwrap();
    // The same payload should come through, without any transformation.
    assert_eq!(json!(forwarded), json!({"b": true}));

    // Both channels should be empty at this point.
    assert!(a_rx.try_recv().is_err());
    assert!(b_rx.try_recv().is_err());
}

#[tokio::test]
async fn test_transformation_string() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = rx.recv().await {
            let out = match x.input {
                TransformerInput::String(input) => json!({"payload": { "got": input }})
                    .as_object()
                    .cloned()
                    .unwrap(),
                _ => unreachable!(),
            };
            x.callback_tx.send(Ok(TransformerOutput::Object(out))).ok();
        }
    });

    let (a_output, mut a_rx) = FakeReceiverOutput::new();
    let state_map = [(
        "transformed".into(),
        IntegrationState {
            verifier: NoVerifier.into(),
            output: Arc::new(Box::new(a_output)),
            transformation: Some(TransformationConfig::Explicit {
                format: TransformerInputFormat::String,
                src: String::from("handler = (x) => ({ payload: { got: x }})"),
            }),
        },
    )]
    .into_iter()
    .collect();
    let state = InternalState::new(state_map, tx);

    let mut app = router().with_state(state);

    let request = Request::builder()
        .uri("/webhook/transformed")
        .method("POST")
        .header("content-type", "text/plain")
        .body(axum::body::Body::from("plain text"))
        .unwrap();

    let response = ServiceExt::<Request<Body>>::ready(&mut app)
        .await
        .unwrap()
        .call(request)
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = a_rx.try_recv().unwrap();
    // The plain text message should have been added in the key "got"
    assert_eq!(json!(forwarded), json!({"got": "plain text"}));

    assert!(a_rx.try_recv().is_err());
}

// Two different bodies - one used during signing, then the other is what we send in the request.
// This should result in a bad response status.
#[tokio::test]
async fn test_forwarding_svix_verification_mismatch() {
    let signed_payload_bytes = serde_json::to_vec(&json!({"a": true})).unwrap();
    let sent_payload_bytes = serde_json::to_vec(&json!({"a": false})).unwrap();

    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let (a_output, mut a_rx) = FakeReceiverOutput::new();

    let webhook = Arc::new(Webhook::new("whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD").unwrap());

    let timestamp = chrono::Utc::now().timestamp();
    let signature = webhook
        .sign("msg_valid", timestamp, &signed_payload_bytes)
        .unwrap();

    let state_map = [(
        "a".into(),
        IntegrationState {
            verifier: SvixVerifier::new(webhook).into(),
            output: Arc::new(Box::new(a_output)),
            transformation: None,
        },
    )]
    .into_iter()
    .collect();
    let state = InternalState::new(state_map, tx);
    let app = router().with_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook/a")
                .method("POST")
                .header("content-type", "application/json")
                .header("svix-id", "msg_valid")
                .header("svix-signature", signature.clone())
                .header("svix-timestamp", format!("{timestamp}"))
                .body(axum::body::Body::from(sent_payload_bytes))
                .unwrap(),
        )
        .await
        .unwrap();

    // Expect a rejection due to signature verification failure.
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    // There should be noting in the channel since the request should _not have been forwarded_.
    assert!(a_rx.try_recv().is_err());
}

#[tokio::test]
async fn test_forwarding_svix_verification_match() {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let (a_output, mut a_rx) = FakeReceiverOutput::new();

    let webhook = Arc::new(Webhook::new("whsec_C2FVsBQIhrscChlQIMV+b5sSYspob7oD").unwrap());

    let payload = json!({"a": true});
    let payload_bytes = serde_json::to_vec(&payload).unwrap();
    let timestamp = chrono::Utc::now().timestamp();
    let signature = webhook
        .sign("msg_valid", timestamp, &payload_bytes)
        .unwrap();

    let state_map = [(
        "a".into(),
        IntegrationState {
            verifier: SvixVerifier::new(webhook).into(),
            output: Arc::new(Box::new(a_output)),
            transformation: None,
        },
    )]
    .into_iter()
    .collect();
    let state = InternalState::new(state_map, tx);
    let app = router().with_state(state);
    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook/a")
                .method("POST")
                .header("content-type", "application/json")
                .header("svix-id", "msg_valid")
                .header("svix-signature", signature.clone())
                .header("svix-timestamp", timestamp.to_string())
                .body(axum::body::Body::from(payload_bytes))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = a_rx.try_recv().unwrap();
    assert_eq!(json!(forwarded), json!({ "a": true }));
}

mod http_output {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicU16, Ordering},
        },
        time::Duration,
    };

    use axum::{
        Router,
        body::{Body, Bytes},
        extract::State,
        http::{HeaderMap, Request, StatusCode},
        routing::post,
    };
    use serde_json::{Value, json};
    use tokio::{net::TcpListener, sync::mpsc, task::JoinHandle, time::timeout};
    use tower::ServiceExt;

    use crate::{
        config::{Config, EitherReceiver},
        webhook_receiver::{router, types::InternalState},
    };

    // Test fixture helpers: real HTTP destination and configured webhook router.
    #[derive(Clone)]
    struct DestinationState {
        status: Arc<AtomicU16>,
        requests: mpsc::UnboundedSender<(HeaderMap, Bytes)>,
    }

    struct Destination {
        url: String,
        status: Arc<AtomicU16>,
        requests: mpsc::UnboundedReceiver<(HeaderMap, Bytes)>,
        task: JoinHandle<()>,
    }

    impl Destination {
        async fn start(status: StatusCode) -> Self {
            let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
            let url = format!("http://{}/forward", listener.local_addr().unwrap());
            let status = Arc::new(AtomicU16::new(status.as_u16()));
            let (tx, requests) = mpsc::unbounded_channel();
            let app =
                Router::new()
                    .route(
                        "/forward",
                        post(
                            |State(state): State<DestinationState>,
                             headers: HeaderMap,
                             body: Bytes| async move {
                                state.requests.send((headers, body)).unwrap();
                                StatusCode::from_u16(state.status.load(Ordering::Relaxed)).unwrap()
                            },
                        ),
                    )
                    .with_state(DestinationState {
                        status: Arc::clone(&status),
                        requests: tx,
                    });
            let task = tokio::spawn(async move {
                axum::serve(listener, app).await.unwrap();
            });
            Self {
                url,
                status,
                requests,
                task,
            }
        }

        fn assert_forwarded(&mut self, payload: &Value, user_agent: &str) {
            let (headers, body) = self.requests.try_recv().unwrap();
            assert_eq!(serde_json::from_slice::<Value>(&body).unwrap(), *payload);
            assert_eq!(headers["content-type"], "application/json");
            assert_eq!(headers["x-bridge-test"], "forwarded");
            assert_eq!(headers["user-agent"], user_agent);
            assert!(matches!(
                self.requests.try_recv(),
                Err(mpsc::error::TryRecvError::Empty)
            ));
        }
    }

    impl Drop for Destination {
        fn drop(&mut self) {
            self.task.abort();
        }
    }

    /// Use the public configuration path so this test cannot substitute a fake output.
    async fn configured_router(url: &str, user_agent: Option<&str>) -> Router {
        let mut headers = json!({"x-bridge-test": "forwarded"});
        if let Some(user_agent) = user_agent {
            headers["User-Agent"] = json!(user_agent);
        }
        let cfg = Config::from_src(
            &json!({
                "receivers": [{
                    "name": "http-output-test",
                    "input": {
                        "type": "webhook",
                        "path_id": "http-output",
                        "verification": {"type": "none"}
                    },
                    "output": {"type": "http", "url": url, "headers": headers}
                }]
            })
            .to_string(),
            None,
        )
        .await
        .unwrap();
        let routes = cfg
            .receivers
            .into_iter()
            .map(|receiver| match EitherReceiver::from(receiver) {
                EitherReceiver::Webhook(cfg) => cfg,
                EitherReceiver::Poller(_) => panic!("expected a webhook receiver"),
            })
            .collect();
        let (tx, _rx) = mpsc::unbounded_channel();
        let state = InternalState::from_receiver_configs(routes, tx)
            .await
            .unwrap();
        router().with_state(state)
    }

    async fn submit(app: Router, payload: &Value) -> StatusCode {
        timeout(
            Duration::from_secs(5),
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/webhook/http-output")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            ),
        )
        .await
        .expect("webhook forwarding timed out")
        .unwrap()
        .status()
    }

    const DEFAULT_USER_AGENT: &str = concat!("Svix-Bridge/", env!("CARGO_PKG_VERSION"));
    // End test fixture helpers.

    #[tokio::test]
    async fn rejects_503_and_accepts_manual_resubmission_after_recovery() {
        let mut destination = Destination::start(StatusCode::SERVICE_UNAVAILABLE).await;
        let app = configured_router(&destination.url, None).await;
        let payload = json!({"event": "order.created", "data": {"id": 42}});

        let rejected_status = submit(app.clone(), &payload).await;
        destination.assert_forwarded(&payload, DEFAULT_USER_AGENT);

        destination
            .status
            .store(StatusCode::NO_CONTENT.as_u16(), Ordering::Relaxed);
        // A second caller submission proves recovery, not an automatic upstream retry.
        let status = submit(app, &payload).await;
        destination.assert_forwarded(&payload, DEFAULT_USER_AGENT);
        assert_eq!(status, StatusCode::NO_CONTENT);
        assert_eq!(
            rejected_status,
            StatusCode::INTERNAL_SERVER_ERROR,
            "destination returned 503"
        );
    }

    #[tokio::test]
    async fn rejects_client_and_server_errors() {
        let mut statuses = Vec::new();
        for downstream in [
            StatusCode::BAD_REQUEST,
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::INTERNAL_SERVER_ERROR,
        ] {
            let mut destination = Destination::start(downstream).await;
            let app = configured_router(&destination.url, None).await;
            let payload = json!({"event": "order.created"});

            let status = submit(app, &payload).await;
            destination.assert_forwarded(&payload, DEFAULT_USER_AGENT);
            statuses.push(status);
        }
        assert_eq!(
            statuses,
            vec![StatusCode::INTERNAL_SERVER_ERROR; 3],
            "destination returned 400, 429, then 500"
        );
    }

    #[tokio::test]
    async fn accepts_successful_responses() {
        for downstream in [StatusCode::OK, StatusCode::CREATED, StatusCode::NO_CONTENT] {
            let mut destination = Destination::start(downstream).await;
            let app = configured_router(&destination.url, None).await;
            let payload = json!({"event": "order.created"});

            let status = submit(app, &payload).await;
            destination.assert_forwarded(&payload, DEFAULT_USER_AGENT);
            assert_eq!(
                status,
                StatusCode::NO_CONTENT,
                "destination returned {downstream}"
            );
        }
    }

    #[tokio::test]
    async fn preserves_configured_user_agent() {
        let mut destination = Destination::start(StatusCode::NO_CONTENT).await;
        let app = configured_router(&destination.url, Some("custom-forwarder/1.0")).await;
        let payload = json!({"event": "order.created"});

        let status = submit(app, &payload).await;
        destination.assert_forwarded(&payload, "custom-forwarder/1.0");
        assert_eq!(status, StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn rejects_transport_errors() {
        // Keep the listener bound while closing connections to avoid a port-reuse race.
        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let url = format!("http://{}/forward", listener.local_addr().unwrap());
        let task = tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                drop(stream);
            }
        });
        let app = configured_router(&url, None).await;

        let status = submit(app, &json!({"event": "order.created"})).await;
        task.abort();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
