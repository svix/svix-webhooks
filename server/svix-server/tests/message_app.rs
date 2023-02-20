// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Tests related to the [`CreateMessageApp`] and the [`CreateMessageEndpoint`]s structs.
use std::time::Duration;

use http::StatusCode;

mod utils;

use svix_server::{
    cfg::{CacheBackend, CacheType, Configuration},
    core::{
        cache::{self, CacheBehavior},
        message_app::AppEndpointKey,
        types::{BaseId, OrganizationId},
    },
    redis::{new_redis_pool, new_redis_pool_clustered, RedisPool},
};
use utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_message, message_in},
    get_default_test_config, start_svix_server_with_cfg_and_org_id, IgnoredResponse, TestReceiver,
};

pub async fn get_pool(cfg: Configuration) -> RedisPool {
    match cfg.cache_type {
        CacheType::RedisCluster => {
            new_redis_pool_clustered(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await
        }
        _ => new_redis_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await,
    }
}

/// Ensures that a deleted application returns `None` when using [`layered_fetch`]
#[tokio::test]
async fn test_app_deletion() {
    dotenv::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading Configuration");
    let org_id = OrganizationId::new(None, None);
    let (client, _jh) =
        start_svix_server_with_cfg_and_org_id(&get_default_test_config(), org_id.clone()).await;

    // Cannot run test using an in-memory cache as we can't invalidate a key from within the test.
    // Ie. the Redis backends all share the same memory when a cache is created in this test. The
    // same is not true for an in-memory map.
    if matches!(cfg.cache_backend(), CacheBackend::Memory) {
        return;
    }

    let mut test_receiver = TestReceiver::start(axum::http::StatusCode::OK);

    let app_id = create_test_app(&client, "TestAppDeletion")
        .await
        .unwrap()
        .id;
    let _ = create_test_endpoint(&client, &app_id, &test_receiver.endpoint)
        .await
        .unwrap();

    let payload = serde_json::json!({"test": "value"});

    create_test_message(&client, &app_id, payload.clone())
        .await
        .unwrap();

    assert_eq!(
        tokio::time::timeout(Duration::from_millis(250), test_receiver.data_recv.recv()).await,
        Ok(Some(payload.clone()))
    );

    client
        .delete::<IgnoredResponse>(&format!("api/v1/app/{app_id}/"), StatusCode::NO_CONTENT)
        .await
        .unwrap();

    // Delete the cached [`CreateMessageApp`] here instead of waiting 30s for it to expire
    let cache = match cfg.cache_backend() {
        CacheBackend::None => cache::none::new(),
        CacheBackend::Redis(dsn) => {
            let mgr = new_redis_pool(dsn, &cfg).await;
            cache::redis::new(mgr)
        }
        CacheBackend::RedisCluster(dsn) => {
            let mgr = new_redis_pool_clustered(dsn, &cfg).await;
            cache::redis::new(mgr)
        }

        // Cannot use memory cache for this test. See the above check.
        CacheBackend::Memory => unreachable!(),
    };

    cache
        .delete(&AppEndpointKey::new(&org_id, &app_id))
        .await
        .unwrap();

    // Assert message creation return a 404 with a deleted application
    client
        .post::<_, IgnoredResponse>(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in("test.event", payload).unwrap(),
            StatusCode::NOT_FOUND,
        )
        .await
        .unwrap();

    // And assert that no message is sent even if it 404s
    assert!(
        tokio::time::timeout(Duration::from_millis(250), test_receiver.data_recv.recv())
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_endp_deletion() {
    dotenv::dotenv().ok();
    let cfg = svix_server::cfg::load().expect("Error loading Configuration");
    let org_id = OrganizationId::new(None, None);
    let (client, _jh) =
        start_svix_server_with_cfg_and_org_id(&get_default_test_config(), org_id.clone()).await;

    // Cannot run test using an in-memory cache as we can't invalidate a key from within the test.
    // Ie. the Redis backends all share the same memory when a cache is created in this test. The
    // same is not true for an in-memory map.
    if matches!(cfg.cache_backend(), CacheBackend::Memory) {
        return;
    }

    let mut test_receiver = TestReceiver::start(axum::http::StatusCode::OK);

    let app_id = create_test_app(&client, "TestAppDeletion")
        .await
        .unwrap()
        .id;
    let endp_id = create_test_endpoint(&client, &app_id, &test_receiver.endpoint)
        .await
        .unwrap()
        .id;

    let payload = serde_json::json!({"test": "value"});

    create_test_message(&client, &app_id, payload.clone())
        .await
        .unwrap();

    assert_eq!(
        tokio::time::timeout(Duration::from_millis(250), test_receiver.data_recv.recv()).await,
        Ok(Some(payload.clone()))
    );

    client
        .delete::<IgnoredResponse>(
            &format!("api/v1/app/{app_id}/endpoint/{endp_id}/"),
            StatusCode::NO_CONTENT,
        )
        .await
        .unwrap();

    // Delete the cached [`CreateMessageApp`] here instead of waiting 30s for it to expire
    let cache = match cfg.cache_backend() {
        CacheBackend::None => cache::none::new(),
        CacheBackend::Redis(dsn) => {
            let mgr = new_redis_pool(dsn, &cfg).await;
            cache::redis::new(mgr)
        }
        CacheBackend::RedisCluster(dsn) => {
            let mgr = new_redis_pool_clustered(dsn, &cfg).await;
            cache::redis::new(mgr)
        }

        // Cannot use memory cache for this test. See the above check.
        CacheBackend::Memory => unreachable!(),
    };

    cache
        .delete(&AppEndpointKey::new(&org_id, &app_id))
        .await
        .unwrap();

    // Assert message creation return a 202 with a deleted endpoint
    client
        .post::<_, IgnoredResponse>(
            &format!("api/v1/app/{app_id}/msg/"),
            message_in("test.event", payload).unwrap(),
            StatusCode::ACCEPTED,
        )
        .await
        .unwrap();

    // But assert that no message is sent as the endpoint no longer exists
    assert!(
        tokio::time::timeout(Duration::from_millis(250), test_receiver.data_recv.recv())
            .await
            .is_err()
    );
}
