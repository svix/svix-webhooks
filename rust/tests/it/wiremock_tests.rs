use svix::api::{MessageListOptions, Svix, SvixOptions};

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

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

    let svx = Svix::new(
        "token".to_string(),
        Some(SvixOptions {
            server_url: Some(mock_server.uri()),
            ..Default::default()
        }),
    );

    svx.message()
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
