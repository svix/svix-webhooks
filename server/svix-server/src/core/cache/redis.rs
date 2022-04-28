// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use axum::async_trait;

use serde_json;

use crate::redis::{PoolLike, SvixRedisPool};

use super::{Cache, CacheBehavior, CacheKey, CacheValue, Error, Result};

pub fn new(redis: SvixRedisPool) -> Cache {
    RedisCache { redis }.into()
}

#[derive(Clone)]
pub struct RedisCache {
    redis: SvixRedisPool,
}

#[async_trait]
impl CacheBehavior for RedisCache {
    async fn get<T: CacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        let mut pool = self.redis.get().await.unwrap();

        let fetched: Option<String> = pool.query_async(redis::Cmd::get(key.as_ref())).await?;

        Ok(fetched
            .map(|json| serde_json::from_str(&json))
            .transpose()?)
    }

    async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        let mut pool = self.redis.get().await?;

        pool.query_async(redis::Cmd::pset_ex(
            key.as_ref(),
            serde_json::to_string(value)?,
            ttl.as_millis().try_into().map_err(|e| {
                Error::Input(format!(
                    "Duration given cannot be converted to usize: {}",
                    e
                ))
            })?,
        ))
        .await
        .map_err(Into::into)
    }

    async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        let mut pool = self.redis.get().await?;

        pool.query_async(redis::Cmd::del(key.as_ref())).await?;

        Ok(())
    }

    async fn set_if_not_exists<T: CacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        let mut pool = self.redis.get().await?;

        let mut cmd = redis::Cmd::set(key.as_ref(), serde_json::to_string(value)?);

        cmd.arg("PX");
        let ttl_as_millis: u64 = ttl.as_millis().try_into().map_err(|e| {
            Error::Input(format!(
                "Duration given cannot be converted to usize: {}",
                e
            ))
        })?;
        cmd.arg(ttl_as_millis);

        cmd.arg("NX");

        let res: Option<()> = pool.query_async(cmd).await?;

        Ok(res.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::{super::kv_def, *};
    use serde::{Deserialize, Serialize};

    use crate::cfg::CacheType;

    // Test structures

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestValA(usize);
    kv_def!(TestKeyA, TestValA);
    impl TestKeyA {
        fn new(id: String) -> TestKeyA {
            TestKeyA(format!("SVIX_TEST_KEY_A_{}", id))
        }
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestValB(String);
    kv_def!(TestKeyB, TestValB);
    impl TestKeyB {
        fn new(id: String) -> TestKeyB {
            TestKeyB(format!("SVIX_TEST_KEY_B_{}", id))
        }
    }

    async fn get_pool(redis_dsn: &str, cache_type: &crate::cfg::CacheType) -> SvixRedisPool {
        match cache_type {
            CacheType::RedisCluster => {
                let mgr = crate::redis::create_redis_pool(redis_dsn, true).await;
                mgr
            }
            _ => {
                let mgr = crate::redis::create_redis_pool(redis_dsn, false).await;
                mgr
            }
        }
    }

    #[tokio::test]
    async fn test_cache_crud_no_ttl() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg.cache_type).await;
        let cache = super::new(redis_pool);

        let (first_key, first_val_a, first_val_b) =
            (TestKeyA::new("1".to_owned()), TestValA(1), TestValA(2));
        let (second_key, second_val_a, second_val_b) = (
            TestKeyB::new("1".to_owned()),
            TestValB("1".to_owned()),
            TestValB("2".to_owned()),
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

        // Read
        assert_eq!(cache.get(&first_key).await.unwrap(), Some(first_val_a));
        assert_eq!(cache.get(&second_key).await.unwrap(), Some(second_val_a));

        // Update (overwrite)
        assert!(cache
            .set(&first_key, &first_val_b, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(cache
            .set(&second_key, &second_val_b, Duration::from_secs(30),)
            .await
            .is_ok());

        // Confirm update
        assert_eq!(cache.get(&first_key).await.unwrap(), Some(first_val_b));
        assert_eq!(cache.get(&second_key).await.unwrap(), Some(second_val_b));

        // Delete
        assert!(cache.delete(&first_key).await.is_ok());
        assert!(cache.delete(&second_key).await.is_ok());

        // Confirm deletion
        assert_eq!(cache.get::<TestValA>(&first_key).await.unwrap(), None);
        assert_eq!(cache.get::<TestValB>(&second_key).await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_cache_ttl() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg.cache_type).await;
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
    async fn test_cache_nx_status() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg.cache_type).await;
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
