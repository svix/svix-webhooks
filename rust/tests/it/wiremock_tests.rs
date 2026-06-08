use serde_json::json;
use svix::api::{MessageIn, MessageListOptions, Svix, SvixOptions};

use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

trait MockServerExt {
    fn svix_client(&self) -> Svix;
}

impl MockServerExt for MockServer {
    fn svix_client(&self) -> Svix {
        Svix::new(
            "token".to_owned(),
            Some(SvixOptions {
                server_url: Some(self.uri()),
                ..Default::default()
            }),
        )
    }
}

#[tokio::test]
async fn test_urlencoded_octothorpe() {
    let mock_server = MockServer::start().await;

    let json_body =
        r#"{"data":[],"done":true,"iterator":"iterator-str","prevIterator":"prevIterator-str"}"#;
    Mock::given(method("GET"))
        .and(path("/api/v1/app/app_id/msg"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json_body))
        .mount(&mock_server)
        .await;

    mock_server
        .svix_client()
        .message()
        .list(
            "app_id".to_string(),
            Some(MessageListOptions {
                tag: Some("test#test".into()),
                ..Default::default()
            }),
        )
        .await
        .unwrap();

    let requests = mock_server
        .received_requests()
        .await
        .expect("we should have sent a request");

    assert_eq!(1, requests.len());
    assert_eq!(Some("tag=test%23test"), requests[0].url.query());
}

#[tokio::test]
async fn test_idempotency_key_is_sent_for_create_request() {
    let mock_server = MockServer::start().await;

    let json_body = r#"{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}"#;
    Mock::given(method("POST"))
        .and(path("/api/v1/app"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json_body))
        .mount(&mock_server)
        .await;

    mock_server
        .svix_client()
        .application()
        .create(svix::api::ApplicationIn::new("test app".to_string()), None)
        .await
        .unwrap();

    let requests = mock_server
        .received_requests()
        .await
        .expect("we should have sent a request");

    assert_eq!(1, requests.len());
    let idempotency_key = requests[0]
        .headers
        .get("idempotency-key")
        .expect("idempotency-key header should be present");
    assert!(
        idempotency_key.to_str().unwrap().starts_with("auto_"),
        "idempotency key should start with 'auto_', got: {idempotency_key:?}"
    );
}

#[tokio::test]
async fn test_client_provided_idempotency_key_is_not_overridden() {
    let mock_server = MockServer::start().await;

    let json_body = r#"{"uid":"unique-identifier","name":"My first application","rateLimit":0,"id":"app_1srOrx2ZWZBpBUvZwXKQmoEYga2","createdAt":"2019-08-24T14:15:22Z","updatedAt":"2019-08-24T14:15:22Z","metadata":{"property1":"string","property2":"string"}}"#;
    Mock::given(method("POST"))
        .and(path("/api/v1/app"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json_body))
        .mount(&mock_server)
        .await;

    let client_provided_key = "test-key-123";
    mock_server
        .svix_client()
        .application()
        .create(
            svix::api::ApplicationIn::new("test app".to_string()),
            Some(svix::api::ApplicationCreateOptions {
                idempotency_key: Some(client_provided_key.to_string()),
            }),
        )
        .await
        .unwrap();

    let requests = mock_server
        .received_requests()
        .await
        .expect("we should have sent a request");

    assert_eq!(1, requests.len());
    let idempotency_key = requests[0]
        .headers
        .get("idempotency-key")
        .expect("idempotency-key header should be present");
    assert_eq!(
        client_provided_key,
        idempotency_key.to_str().unwrap(),
        "client provided idempotency key should not be overridden"
    );
}

#[tokio::test]
async fn test_unknown_keys_are_ignored() {
    let mock_server = MockServer::start().await;

    let json_body =
        r#"{"data":[],"done":true,"iterator":null,"prevIterator":null,"extra-key":"ignored"}"#;
    Mock::given(method("GET"))
        .and(path("/api/v1/app"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    mock_server
        .svix_client()
        .application()
        .list(None)
        .await
        .unwrap();

    mock_server.verify().await;
}

#[tokio::test]
async fn test_cmg_with_content_default() {
    let mock_server = MockServer::start().await;

    let app_id = "app_1srOrx2ZWZBpBUvZwXKQmoEYga2";
    let event_type = "user.signup";
    let response_body = json!({
        "channels": null,
        "deliverAt": null,
        "eventId": null,
        "eventType": event_type,
        "id": "msg_2srOrx2ZWZBpBUvZwXKQmoEYga2",
        "payload": { "m": "FILTERED" },
        "tags": null,
        "timestamp": "2026-06-08T09:25:17.864Z"
    });
    Mock::given(method("POST"))
        .and(path(format!("/api/v1/app/{app_id}/msg")))
        .and(query_param("with_content", "false"))
        .respond_with(ResponseTemplate::new(202).set_body_json(response_body))
        .expect(1)
        .mount(&mock_server)
        .await;

    let payload = json!({
        "email": "test@example.com",
        "type": event_type,
        "username": "test_user",
    });
    let response = mock_server
        .svix_client()
        .message()
        .create(
            app_id.to_owned(),
            MessageIn::new(event_type.to_owned(), payload.clone()),
            None,
        )
        .await
        .unwrap();

    assert_eq!(response.payload, payload);
    mock_server.verify().await;
}
