// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use axum::async_trait;
use redis::AsyncCommands as _;

use super::{Cache, CacheBehavior, CacheKey, Error, Result};
use crate::redis::RedisPool;

pub fn new(redis: RedisPool) -> Cache {
    RedisCache { redis }.into()
}

#[derive(Clone)]
pub struct RedisCache {
    redis: RedisPool,
}

#[async_trait]
impl CacheBehavior for RedisCache {
    fn should_retry(&self, e: &Error) -> bool {
        matches!(e, Error::Pool(_) | Error::Database(_))
    }

    async fn get_raw(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let mut pool = self.redis.get().await?;

        let fetched: Option<Vec<u8>> = pool.get(key).await?;

        Ok(fetched)
    }

    async fn set_raw(&self, key: &[u8], value: &[u8], ttl: Duration) -> Result<()> {
        let mut pool = self.redis.get().await?;

        pool.pset_ex(
            key,
            value,
            ttl.as_millis().try_into().map_err(|e| {
                Error::Input(format!("Duration given cannot be converted to usize: {e}"))
            })?,
        )
        .await
        .map_err(Into::into)
    }

    async fn set_raw_if_not_exists(&self, key: &[u8], value: &[u8], ttl: Duration) -> Result<bool> {
        let mut pool = self.redis.get().await?;

        let mut cmd = redis::Cmd::set(key, value);

        cmd.arg("PX");
        let ttl_as_millis: u64 = ttl.as_millis().try_into().map_err(|e| {
            Error::Input(format!("Duration given cannot be converted to usize: {e}"))
        })?;
        cmd.arg(ttl_as_millis);

        cmd.arg("NX");

        let res: Option<()> = pool.query_async(cmd).await?;

        Ok(res.is_some())
    }

    async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        let mut pool = self.redis.get().await?;

        pool.del(key.as_ref()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::{
        super::{kv_def, string_kv_def, CacheValue},
        *,
    };
    use crate::cfg::CacheType;

    // Test structures

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestValA(usize);
    kv_def!(TestKeyA, TestValA);
    impl TestKeyA {
        fn new(id: String) -> TestKeyA {
            TestKeyA(format!("SVIX_TEST_KEY_A_{id}"))
        }
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestValB(String);
    kv_def!(TestKeyB, TestValB);
    impl TestKeyB {
        fn new(id: String) -> TestKeyB {
            TestKeyB(format!("SVIX_TEST_KEY_B_{id}"))
        }
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct StringTestVal(String);
    string_kv_def!(StringTestKey);
    impl StringTestKey {
        fn new(id: String) -> StringTestKey {
            StringTestKey(format!("SVIX_TEST_KEY_STRING_{id}"))
        }
    }

    async fn get_pool(redis_dsn: &str, cfg: &crate::cfg::Configuration) -> RedisPool {
        match cfg.cache_type {
            CacheType::RedisCluster => crate::redis::new_redis_pool_clustered(redis_dsn, cfg).await,
            CacheType::Redis => crate::redis::new_redis_pool(redis_dsn, cfg).await,
            _ => panic!(
                "This test should only be run when redis is configured as the cache provider"
            ),
        }
    }

    #[tokio::test]
    // run with `cargo test -- --ignored redis` only when redis is up and configured
    #[ignore]
    async fn test_cache_crud_no_ttl() {
        dotenvy::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await;
        let cache = super::new(redis_pool);

        let (first_key, first_val_a, first_val_b) =
            (TestKeyA::new("1".to_owned()), TestValA(1), TestValA(2));
        let (second_key, second_val_a, second_val_b) = (
            TestKeyB::new("1".to_owned()),
            TestValB("1".to_owned()),
            TestValB("2".to_owned()),
        );
        let (third_key, third_val_a, third_val_b) = (
            StringTestKey::new("1".to_owned()),
            "1".to_owned(),
            "2".to_owned(),
        );

        // Create
        assert!(cache
            .set(&first_key, &first_val_a, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(cache
            .set(&second_key, &second_val_a, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(cache
            .set_string(&third_key, &third_val_a, Duration::from_secs(30),)
            .await
            .is_ok());

        // Read
        assert_eq!(cache.get(&first_key).await.unwrap(), Some(first_val_a));
        assert_eq!(cache.get(&second_key).await.unwrap(), Some(second_val_a));
        assert_eq!(
            cache.get_string(&third_key).await.unwrap(),
            Some(third_val_a)
        );

        // Update (overwrite)
        assert!(cache
            .set(&first_key, &first_val_b, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(cache
            .set(&second_key, &second_val_b, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(cache
            .set_string(&third_key, &third_val_b, Duration::from_secs(30),)
            .await
            .is_ok());

        // Confirm update
        assert_eq!(cache.get(&first_key).await.unwrap(), Some(first_val_b));
        assert_eq!(cache.get(&second_key).await.unwrap(), Some(second_val_b));
        assert_eq!(
            cache.get_string(&third_key).await.unwrap(),
            Some(third_val_b)
        );

        // Delete
        assert!(cache.delete(&first_key).await.is_ok());
        assert!(cache.delete(&second_key).await.is_ok());
        assert!(cache.delete(&third_key).await.is_ok());

        // Confirm deletion
        assert_eq!(cache.get::<TestValA>(&first_key).await.unwrap(), None);
        assert_eq!(cache.get::<TestValB>(&second_key).await.unwrap(), None);
        assert_eq!(cache.get_string(&third_key).await.unwrap(), None);
    }

    #[tokio::test]
    #[ignore]
    async fn test_cache_ttl() {
        dotenvy::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await;
        let cache = super::new(redis_pool);

        let key = TestKeyA::new("key".to_owned());

        assert!(cache
            .set(&key, &TestValA(1), Duration::from_secs(1),)
            .await
            .is_ok());
        tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
        assert_eq!(cache.get::<TestValA>(&key).await.unwrap(), None);
    }

    #[tokio::test]
    #[ignore]
    async fn test_cache_nx_status() {
        dotenvy::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await;
        let cache = super::new(redis_pool);

        let key = TestKeyA::new("nx_status_test_key".to_owned());

        assert!(cache
            .set_if_not_exists(&key, &TestValA(1), Duration::from_secs(30),)
            .await
            .unwrap());
        assert_eq!(cache.get(&key).await.unwrap(), Some(TestValA(1)));

        assert!(!cache
            .set_if_not_exists(&key, &TestValA(2), Duration::from_secs(30),)
            .await
            .unwrap());
        assert_eq!(cache.get(&key).await.unwrap(), Some(TestValA(1)));

        assert!(cache.delete(&key).await.is_ok());
    }
}
