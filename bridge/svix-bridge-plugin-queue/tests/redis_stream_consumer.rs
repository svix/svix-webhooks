//! Use the `testing-docker-compose.yml` in the repo root to run the dependencies for testing,
//! including Redis.

use std::time::Duration;

use redis::{AsyncCommands, Client};
use serde_json::json;
use svix_bridge_plugin_queue::{config::RedisInputOpts, RedisConsumerPlugin};
use svix_bridge_types::{
    svix::api::MessageIn, CreateMessageRequest, SenderInput, SenderOutputOpts, SvixOptions,
    SvixSenderOutputOpts, TransformationConfig, TransformerInput, TransformerInputFormat,
    TransformerJob, TransformerOutput,
};
use wiremock::matchers::{body_partial_json, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn get_test_plugin(
    svix_url: String,
    queue_key: String,
    use_transformation: Option<TransformerInputFormat>,
) -> RedisConsumerPlugin {
    RedisConsumerPlugin::new(
        "test".into(),
        RedisInputOpts {
            dsn: "redis://localhost/".to_owned(),
            max_connections: 8,
            reinsert_on_nack: false,
            queue_key,
            consumer_group: "test_cg".to_owned(),
            consumer_name: "test_cn".to_owned(),
        },
        use_transformation.map(|format| TransformationConfig::Explicit {
            format,
            src: String::from("function handle(x) { return x; }"),
        }),
        SenderOutputOpts::Svix(SvixSenderOutputOpts {
            token: "xxxx".to_string(),
            options: Some(SvixOptions {
                server_url: Some(svix_url),
                ..Default::default()
            }),
        }),
    )
}

async fn redis_connection() -> Client {
    Client::open("redis://localhost/").unwrap()
}

async fn create_test_stream(client: &Client) -> String {
    let name: String = std::iter::repeat_with(fastrand::alphanumeric)
        .take(8)
        .collect();

    let mut conn = client.get_async_connection().await.unwrap();

    let _: () = conn
        .xgroup_create_mkstream(&name, "test_cg", 0i8)
        .await
        .unwrap();

    name
}

async fn delete_test_stream(client: &Client, key: &str) {
    let mut conn = client.get_async_connection().await.unwrap();
    let _: () = conn.del(key).await.unwrap();
}

async fn publish(client: &Client, key: &str, payload: &str) {
    let mut conn = client.get_async_connection().await.unwrap();

    let _: () = conn.xadd(key, "*", &[("payload", payload)]).await.unwrap();
}

/// General "pause while we wait for messages to travel" beat. If you're seeing flakes, bump this up.
const WAIT_MS: u64 = 250;

/// Push a msg on the queue.
/// Check to see if the svix server sees a request.
#[tokio::test]
async fn test_consume_ok() {
    let client = redis_connection().await;
    let key = create_test_stream(&client).await;

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            "hi": "there",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), key.clone(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        post_options: None,
    };

    publish(&client, &key, &serde_json::to_string(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();

    delete_test_stream(&client, &key).await;
}

/// Push a msg on the queue.
/// Check to see if the svix server sees a request, but this time transform the payload.
#[tokio::test]
async fn test_consume_transformed_json_ok() {
    let client = redis_connection().await;
    let key = create_test_stream(&client).await;

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .and(body_partial_json(json!({ "payload": { "good": "bye" } })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            // The adjustment made via the transformation...
            "good": "bye",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let mut plugin = get_test_plugin(
        mock_server.uri(),
        key.clone(),
        Some(TransformerInputFormat::Json),
    );
    let (transformer_tx, mut transformer_rx) =
        tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = transformer_rx.recv().await {
            let mut out = match x.input {
                TransformerInput::JSON(input) => input.as_object().unwrap().clone(),
                _ => unreachable!(),
            };
            // Prune out the "hi" key.
            out["message"]["payload"]
                .as_object_mut()
                .unwrap()
                .remove("hi");
            // Add the "good" key.
            out["message"]["payload"]["good"] = json!("bye");
            x.callback_tx.send(Ok(TransformerOutput::Object(out))).ok();
        }
    });
    plugin.set_transformer(Some(transformer_tx));

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    let msg = CreateMessageRequest {
        app_id: "app_1234".into(),
        message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
        post_options: None,
    };

    publish(&client, &key, &serde_json::to_string(&msg).unwrap()).await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();

    delete_test_stream(&client, &key).await;
}

// FIXME: can't support raw payload with the current redis backend impl as it requires json
//   internally. Try again when we switch to `omniqueue`.
#[ignore]
#[tokio::test]
async fn test_consume_transformed_string_ok() {
    let client = redis_connection().await;
    let key = create_test_stream(&client).await;

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .and(body_partial_json(
            json!({ "payload": { "hello": "world" } }),
        ))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
          "eventType": "testing.things",
          "payload": {
            // The adjustment made via the transformation...
            "hello": "world",
          },
          "id": "msg_xxxx",
          "timestamp": "2023-04-25T00:00:00Z"
        })))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let mut plugin = get_test_plugin(
        mock_server.uri(),
        key.clone(),
        Some(TransformerInputFormat::String),
    );
    let (transformer_tx, mut transformer_rx) =
        tokio::sync::mpsc::unbounded_channel::<TransformerJob>();
    let _handle = tokio::spawn(async move {
        while let Some(x) = transformer_rx.recv().await {
            let input = match x.input {
                TransformerInput::String(input) => input,
                _ => unreachable!(),
            };
            // Build a create-message-compatible object, using the string input as a field in the payload.
            let out = json!({
                "appId": "app_1234",
                "message": {
                    "eventType": "testing.things",
                    "payload": {
                        "hello": input,
                    }
                }
            });
            x.callback_tx
                .send(Ok(TransformerOutput::Object(
                    out.as_object().unwrap().clone(),
                )))
                .ok();
        }
    });
    plugin.set_transformer(Some(transformer_tx));

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(&client, &key, "world").await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    handle.abort();

    delete_test_stream(&client, &key).await;
}

#[tokio::test]
async fn test_missing_app_id_nack() {
    let client = redis_connection().await;
    let key = create_test_stream(&client).await;

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), key.clone(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &key,
        &serde_json::to_string(&json!({
            // No app id
            "message": {
                "eventType": "testing.things",
                "payload": {
                    "hi": "there",
                }
            },

        }))
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;
    handle.abort();

    delete_test_stream(&client, &key).await
}

#[tokio::test]
async fn test_missing_event_type_nack() {
    let client = redis_connection().await;
    let key = create_test_stream(&client).await;

    let mock_server = MockServer::start().await;
    let mock = Mock::given(method("POST"))
        // The response doesn't really matter, but we need to define it to be able to `expect(0)`.
        .respond_with(ResponseTemplate::new(400))
        .named("create_message")
        // No requests should be made when the event type or app id are missing.
        .expect(0);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), key.clone(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });

    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &key,
        &serde_json::to_string(&json!({
            "appId": "app_1234",
            "message": {
                // No event type
                "payload": {
                    "hi": "there",
                }
            },
        }))
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;
    handle.abort();

    delete_test_stream(&client, &key).await
}

/// Check that the plugin keeps running when it can't send a message to svix
#[tokio::test]
async fn test_consume_svix_503() {
    let client = redis_connection().await;
    let key = create_test_stream(&client).await;

    let mock_server = MockServer::start().await;
    // The mock will make asserts on drop (i.e. when the body of the test is returning).
    // The `expect` call should ensure we see exactly 1 POST request.
    // <https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect>
    let mock = Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503))
        .named("create_message")
        .expect(1);
    mock_server.register(mock).await;

    let plugin = get_test_plugin(mock_server.uri(), key.clone(), None);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &key,
        &serde_json::to_string(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
            post_options: None,
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    assert!(!handle.is_finished());
    handle.abort();

    delete_test_stream(&client, &key).await
}

/// Check that the plugin keeps running when it can't send a message to svix because idk, the servers are all offline??
#[tokio::test]
async fn test_consume_svix_offline() {
    let client = redis_connection().await;
    let key = create_test_stream(&client).await;

    let mock_server = MockServer::start().await;

    let plugin = get_test_plugin(mock_server.uri(), key.clone(), None);

    // bye-bye svix...
    drop(mock_server);

    let handle = tokio::spawn(async move {
        let fut = plugin.run();
        fut.await
    });
    // Wait for the consumer to connect
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    publish(
        &client,
        &key,
        &serde_json::to_string(&CreateMessageRequest {
            app_id: "app_1234".into(),
            message: MessageIn::new("testing.things".into(), json!({"hi": "there"})),
            post_options: None,
        })
        .unwrap(),
    )
    .await;

    // Wait for the consumer to consume.
    tokio::time::sleep(Duration::from_millis(WAIT_MS)).await;

    assert!(!handle.is_finished());
    handle.abort();

    delete_test_stream(&client, &key).await
}
