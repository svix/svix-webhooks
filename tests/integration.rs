use std::time::Duration;

use coyote_client::{
    CoyoteClient, CoyoteOptions,
    models::{CacheDeleteIn, CacheGetIn, CacheSetIn, KvDeleteIn, KvGetIn, KvSetIn},
};

fn client() -> CoyoteClient {
    let token = std::env::var("COYOTE_TOKEN").expect("COYOTE_TOKEN must be set");
    let server_url = std::env::var("COYOTE_SERVER_URL").expect("COYOTE_SERVER_URL must be set");
    let options = CoyoteOptions {
        server_url: Some(server_url),
        ..Default::default()
    };
    CoyoteClient::new(token, Some(options))
}

#[tokio::test]
#[ignore]
async fn test_health_ping() {
    let client = client();
    client.health().ping().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn test_kv_set_get_delete() {
    let client = client();
    let key = "rust-integration-kv-key".to_string();
    let value = b"rust-integration-kv-value".to_vec();

    // Set
    let set_resp = client
        .kv()
        .set(key.clone(), KvSetIn::new(value.clone()))
        .await
        .unwrap();
    assert!(set_resp.success);

    // Get
    let get_resp = client.kv().get(key.clone(), KvGetIn::new()).await.unwrap();
    assert_eq!(get_resp.value, Some(value));

    // Delete
    let del_resp = client
        .kv()
        .delete(key.clone(), KvDeleteIn::new())
        .await
        .unwrap();
    assert!(del_resp.success);

    // Verify deleted
    let get_resp = client.kv().get(key, KvGetIn::new()).await.unwrap();
    assert_eq!(get_resp.value, None);
}

#[tokio::test]
#[ignore]
async fn test_cache_set_get_delete() {
    let client = client();
    let key = "rust-integration-cache-key".to_string();
    let value = b"rust-integration-cache-value".to_vec();

    // Set (ttl_ms = 60000)
    client
        .cache()
        .set(
            key.clone(),
            CacheSetIn::new(value.clone(), Duration::from_secs(60)),
        )
        .await
        .unwrap();

    // Get
    let get_resp = client
        .cache()
        .get(key.clone(), CacheGetIn::new())
        .await
        .unwrap();
    assert_eq!(get_resp.value, Some(value));

    // Delete
    let del_resp = client
        .cache()
        .delete(key.clone(), CacheDeleteIn::new())
        .await
        .unwrap();
    assert!(del_resp.success);

    // Verify deleted
    let get_resp = client.cache().get(key, CacheGetIn::new()).await.unwrap();
    assert_eq!(get_resp.value, None);
}
