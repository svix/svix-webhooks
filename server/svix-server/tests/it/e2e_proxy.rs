use std::time::Duration;

use http::StatusCode;
use serde::de::IgnoredAny;
use svix_server::cfg::{ProxyAddr, ProxyConfig};
use tokio::time::timeout;

use crate::utils::{
    common_calls::{create_test_app, create_test_endpoint, message_in},
    get_default_test_config, TestClient, TestReceiver,
};

#[ignore] // works with microsocks running at the specified address
#[tokio::test]
async fn test_message_delivery_via_socks5() {
    use crate::utils::start_svix_server_with_cfg;

    let mut cfg = get_default_test_config();
    cfg.proxy_config = Some(socks_proxy_config());
    let (client, _) = start_svix_server_with_cfg(&cfg).await;
    run_proxy_test(&client).await;
}

fn socks_proxy_config() -> ProxyConfig {
    ProxyConfig {
        addr: ProxyAddr::new("socks5://localhost:1080").unwrap(),
    }
}

#[ignore] // works with tinyproxy running at the specified address
#[tokio::test]
async fn test_message_delivery_via_http_proxy() {
    use crate::utils::start_svix_server_with_cfg;

    let mut cfg = get_default_test_config();
    cfg.proxy_config = Some(http_proxy_config());
    let (client, _) = start_svix_server_with_cfg(&cfg).await;
    run_proxy_test(&client).await;
}

fn http_proxy_config() -> ProxyConfig {
    ProxyConfig {
        addr: ProxyAddr::new("http://localhost:8888").unwrap(),
    }
}

async fn run_proxy_test(client: &TestClient) {
    let mut receiver = TestReceiver::start(StatusCode::OK);

    let app_id = create_test_app(client, "proxyTest").await.unwrap().id;
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
