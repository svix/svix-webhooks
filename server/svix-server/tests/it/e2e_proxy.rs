use std::time::Duration;

use http::StatusCode;
use serde::de::IgnoredAny;
use svix_server::cfg::{ProxyAddr, ProxyConfig};
use tokio::time::timeout;

use crate::utils::{
    common_calls::{create_test_app, create_test_endpoint, message_in},
    get_default_test_config, TestClient, TestReceiver,
};

// I could not get it to work with a dockerized socks5 proxy yet
#[ignore]
#[tokio::test]
async fn test_message_delivery_via_socks5() {
    use crate::utils::start_svix_server_with_cfg;

    let mut cfg = get_default_test_config();
    cfg.proxy_config = Some(proxy_config());
    let (client, _) = start_svix_server_with_cfg(&cfg).await;
    run_socks5_test(&client).await;
}

fn proxy_config() -> ProxyConfig {
    ProxyConfig {
        addr: ProxyAddr::new("socks5://localhost:1080").unwrap(),
    }
}

async fn run_socks5_test(client: &TestClient) {
    let mut receiver = TestReceiver::start(StatusCode::OK);

    let app_id = create_test_app(client, "kafkaSinkTest").await.unwrap().id;
    create_test_endpoint(client, &app_id, &receiver.endpoint)
        .await
        .unwrap();

    let msg_payload = serde_json::json!({ "test": "value" });

    let _: IgnoredAny = client
        .post(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, msg_payload.clone()).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    let received_payload = timeout(Duration::from_secs(2), receiver.data_recv.recv())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(received_payload, msg_payload);
}
