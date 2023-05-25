use super::router;
use crate::webhook_receiver::{
    types::{IntegrationState, InternalState},
    verification::{NoVerifier, SvixVerifier},
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use std::sync::Arc;
use svix_bridge_types::{
    async_trait, svix::webhooks::Webhook, JsObject, JsReturn, ReceiverOutput, TransformerJob,
};
use tower::Service;
use tower::ServiceExt;

struct FakeReceiverOutput {
    tx: tokio::sync::mpsc::UnboundedSender<JsObject>,
}

impl FakeReceiverOutput {
    pub fn new() -> (Self, tokio::sync::mpsc::UnboundedReceiver<JsObject>) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        (Self { tx }, rx)
    }
}

#[async_trait]
impl ReceiverOutput for FakeReceiverOutput {
    fn name(&self) -> &str {
        "fake output"
    }

    async fn handle(&self, payload: JsObject) -> std::io::Result<()> {
        self.tx
            .send(payload)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
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
                .body(serde_json::to_vec(&json!({"a": true})).unwrap().into())
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
        .body(serde_json::to_vec(&json!({"a": true})).unwrap().into())
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
        .body(serde_json::to_vec(&json!({"b": true})).unwrap().into())
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

/// Registers 2 receivers, one with a transformation and one without.Sends 1 request to each.
#[tokio::test]
async fn test_transformation() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = rx.recv().await {
            let mut out = x.payload;
            out.insert("__TRANSFORMED__".into(), json!(true));
            x.callback_tx.send(Ok(JsReturn::Object(out))).ok();
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
                transformation: Some("handler = (x) => ({ __TRANSFORMED__: true, ...x})".into()),
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
        .body(serde_json::to_vec(&json!({"a": true})).unwrap().into())
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
        .body(serde_json::to_vec(&json!({"b": true})).unwrap().into())
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
                .header("svix-timestamp", &format!("{timestamp}"))
                .body(sent_payload_bytes.into())
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
                .header("svix-timestamp", &format!("{timestamp}"))
                .body(payload_bytes.into())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let forwarded = a_rx.try_recv().unwrap();
    assert_eq!(json!(forwarded), json!({"a": true}));
}

/// Simulated queue failure. Assuming the [`ReceiverOutput`] implementation has proper error
/// handling and actually gives an `Err` when the queue publish fails, this demonstrates how the
/// HTTP handler behaves.
#[tokio::test]
async fn test_forwarding_error_is_500() {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let (a_output, mut a_rx) = FakeReceiverOutput::new();

    let payload = json!({"a": true});
    let payload_bytes = serde_json::to_vec(&payload).unwrap();

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

    // Close the channel held by the ReceiverOutput to simulate a queue failure.
    a_rx.close();
    // The channel should be empty.
    assert!(a_rx.try_recv().is_err());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/webhook/a")
                .method("POST")
                .header("content-type", "application/json")
                .body(payload_bytes.into())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_non_object_body_is_400() {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let (a_output, _a_rx) = FakeReceiverOutput::new();

    // We require payloads to be an object (though this requirement could be relaxed now that we
    // have transformations).
    let payload = json!([1, 2, 3]);
    let payload_bytes = serde_json::to_vec(&payload).unwrap();

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
                .body(payload_bytes.into())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_non_json_body_is_400() {
    let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
    let (a_output, _a_rx) = FakeReceiverOutput::new();

    let payload_bytes = b"not a json body".to_vec();

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
                .header("content-type", "text/plain")
                .body(payload_bytes.into())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
