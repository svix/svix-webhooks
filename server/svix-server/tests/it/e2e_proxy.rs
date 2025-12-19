use std::{
    collections::HashSet,
    net::TcpListener,
    sync::{Arc, Mutex},
    time::Duration,
};

use http::StatusCode;
use serde::de::IgnoredAny;
use svix_server::{
    cfg::{ProxyAddr, ProxyBypassCfg, ProxyConfig},
    v1::endpoints::message::MessageIn,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

use crate::utils::{
    common_calls::{create_test_app, create_test_endpoint, message_in},
    get_default_test_config, start_svix_server_with_cfg, TestClient, TestReceiver,
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
        noproxy: None,
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
        noproxy: None,
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

// This doesn't actually handle requests successfully, but it does allow us
// to see which hostnames are requested of it.
struct MockProxyServer {
    matched_hosts: Arc<Mutex<HashSet<String>>>,
    addr: String,
    variant: MockProxyVariant,
}

enum MockProxyVariant {
    Http,
    Socks5,
}

impl MockProxyServer {
    pub fn new(variant: MockProxyVariant) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let listener = tokio::net::TcpListener::from_std(listener).unwrap();
        let addr = match variant {
            MockProxyVariant::Http => {
                format!("http://{}", listener.local_addr().unwrap())
            }
            MockProxyVariant::Socks5 => {
                format!("socks5://{}", listener.local_addr().unwrap())
            }
        };
        let matched_hosts = Arc::new(Mutex::new(HashSet::new()));

        match variant {
            MockProxyVariant::Http => {
                tokio::spawn(Self::http_listener(listener, matched_hosts.clone()))
            }
            MockProxyVariant::Socks5 => {
                tokio::spawn(Self::socks5_listener(listener, matched_hosts.clone()))
            }
        };

        Self {
            matched_hosts,
            addr,
            variant,
        }
    }

    pub async fn http_listener(
        listener: tokio::net::TcpListener,
        matched_hosts: Arc<Mutex<HashSet<String>>>,
    ) {
        loop {
            let (mut stream, _addr) = listener.accept().await.unwrap();
            let matched_hosts = matched_hosts.clone();

            tokio::spawn(async move {
                let mut buffer = [0; 512];

                if let Ok(size) = stream.read(&mut buffer).await {
                    if size == 0 {
                        return;
                    }
                    let request = String::from_utf8_lossy(&buffer[..size]);
                    if let Some(host) = request
                        .strip_prefix("CONNECT ")
                        .and_then(|s| s.split(' ').next())
                        .and_then(|s| s.strip_suffix(":443"))
                    {
                        let mut guard = matched_hosts.lock().unwrap();
                        guard.insert(host.to_string());
                    }
                }
            });
        }
    }

    pub async fn socks5_listener(
        listener: tokio::net::TcpListener,
        matched_hosts: Arc<Mutex<HashSet<String>>>,
    ) {
        use socks5_proto::{
            handshake::{
                Method as HandshakeMethod, Request as HandshakeRequest,
                Response as HandshakeResponse,
            },
            Address, Reply, Request as SocksRequest, Response as SocksResponse,
        };
        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(v) => v,
                Err(_) => continue,
            };

            let matched_hosts = matched_hosts.clone();

            tokio::spawn(async move {
                let hs_req = match HandshakeRequest::read_from(&mut stream).await {
                    Ok(req) => req,
                    Err(_) => {
                        return;
                    }
                };

                if hs_req.methods.contains(&HandshakeMethod::NONE) {
                    if HandshakeResponse::new(HandshakeMethod::NONE)
                        .write_to(&mut stream)
                        .await
                        .is_err()
                    {
                        return;
                    }
                } else {
                    let _ = HandshakeResponse::new(HandshakeMethod::UNACCEPTABLE)
                        .write_to(&mut stream)
                        .await;
                    return;
                }

                let Ok(socks_req) = SocksRequest::read_from(&mut stream).await else {
                    return;
                };

                let host = match &socks_req.address {
                    Address::SocketAddress(socket_addr) => socket_addr.ip().to_string(),
                    Address::DomainAddress(domain_bytes, _port) => {
                        String::from_utf8_lossy(domain_bytes).to_string()
                    }
                };
                if !host.is_empty() {
                    let mut guard = matched_hosts.lock().unwrap();
                    guard.insert(host);
                }

                let abort_resp =
                    SocksResponse::new(Reply::ConnectionNotAllowed, Address::unspecified());
                let _ = abort_resp.write_to(&mut stream).await;
                let _ = stream.shutdown().await;
            });
        }
    }

    pub fn matches(&self) -> HashSet<String> {
        let guard = self.matched_hosts.lock().unwrap();
        println!("************ MATCHES {guard:?}");
        guard.clone()
    }
}

#[tokio::test]
async fn test_http_proxy_exceptions() {
    let listener = MockProxyServer::new(MockProxyVariant::Http);
    test_proxy_exceptions(listener).await
}

#[tokio::test]
async fn test_socks5_proxy_exceptions() {
    let listener = MockProxyServer::new(MockProxyVariant::Socks5);
    test_proxy_exceptions(listener).await
}

async fn test_proxy_exceptions(listener: MockProxyServer) {
    let mut cfg = get_default_test_config();
    cfg.proxy_config = Some(ProxyConfig {
        addr: ProxyAddr::new(listener.addr.clone()).unwrap(),
        noproxy: Some(ProxyBypassCfg("10.0.0.0/8, 8.8.8.8, 0ec2:1652:6021:693b:f928:565d:5a0e:de9f, www.svix.com, .google.com".to_owned())),
    });
    cfg.retry_schedule = vec![];

    let (client, _) = start_svix_server_with_cfg(&cfg).await;

    let app_id = create_test_app(&client, "proxyTest").await.unwrap().id;

    // Note: Real hostnames here are not ideal, but difficult to test SOCKS without valid DNS entries:
    create_test_endpoint(&client, &app_id, "https://www.svix.com")
        .await
        .unwrap();

    create_test_endpoint(&client, &app_id, "https://play.svix.com")
        .await
        .unwrap();

    create_test_endpoint(&client, &app_id, "https://www.google.com")
        .await
        .unwrap();

    create_test_endpoint(&client, &app_id, "https://google.com")
        .await
        .unwrap();

    create_test_endpoint(&client, &app_id, "https://8.8.8.8")
        .await
        .unwrap();

    create_test_endpoint(&client, &app_id, "https://8.8.4.4")
        .await
        .unwrap();

    create_test_endpoint(&client, &app_id, "https://10.0.0.1")
        .await
        .unwrap();

    create_test_endpoint(
        &client,
        &app_id,
        "https://[0ec2:1652:6021:693b:f928:565d:5a0e:de9f]",
    )
    .await
    .unwrap();

    client
        .post::<MessageIn, serde_json::Value>(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in(&app_id, serde_json::json!({ "test": "value" })).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    assert!(listener.matches().contains("8.8.4.4"));
    assert!(!listener.matches().contains("8.8.8.8"));
    assert!(!listener.matches().contains("10.0.0.1"));
    assert!(!listener
        .matches()
        .contains("[0ec2:1652:6021:693b:f928:565d:5a0e:de9f]"));
    match listener.variant {
        MockProxyVariant::Http => {
            assert!(listener.matches().contains("play.svix.com"));

            assert!(!listener.matches().contains("www.svix.com"));
            assert!(!listener.matches().contains("www.google.com"));
            assert!(!listener.matches().contains("google.com"));

            assert_eq!(listener.matches().len(), 2);
        }
        MockProxyVariant::Socks5 => {
            // We can't assert hostnames here b/c DNS is resolved before
            // calling the proxy.

            // This is a very weak assertion, but the "insta-retries" that our
            // client does mean that occasionally the same site resolve to
            // different IPs that populate the match list separately:
            assert!(listener.matches().len() >= 2);
        }
    }
}
