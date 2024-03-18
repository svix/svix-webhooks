use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use svix_bridge_types::{
    async_trait, svix::webhooks::Webhook, ForwardRequest, ReceiverOutput, TransformationConfig,
    TransformerInput, TransformerInputFormat, TransformerJob, TransformerOutput,
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

    async fn handle(&self, request: ForwardRequest) -> std::io::Result<()> {
        self.tx.send(request.payload).unwrap();
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

/// Registers 2 receivers, one with a transformation and one without. Sends 1 request to each.
#[tokio::test]
async fn test_transformation_json() {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = rx.recv().await {
            let mut input = match x.input {
                TransformerInput::JSON(input) => input.as_object().unwrap().clone(),
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
        .body("plain text".as_bytes().into())
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
