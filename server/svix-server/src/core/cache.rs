use std::time::Duration;

use bb8::{ManageConnection, Pool};

use redis::{aio::ConnectionLike, AsyncCommands, RedisError};
use serde::{de::DeserializeOwned, Serialize};

/// Errors internal to the cache
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error deserializing Redis value")]
    Deserialization(#[from] serde_json::error::Error),

    #[error("Redis pool error")]
    Pool(#[from] bb8::RunError<RedisError>),

    #[error("Redis databse error")]
    Database(#[from] RedisError),

    #[error("input error: {0}")]
    Input(String),
}
type Result<T> = std::result::Result<T, Error>;

/// A valid key value for the cache -- usually just a wrapper around a [`String`]
pub trait CacheKey: AsRef<str> {
    const PREFIX_CACHE: &'static str = "SVIX_CACHE";
}
/// Any (de)serializable structure usuable as a value in the cache -- it is associated with a
/// given key type to ensure type checking on creation or reading of values from the cache
pub trait CacheValue: DeserializeOwned + Serialize {
    type Key: CacheKey;
}

/// A macro that creates a [`CacheKey`] and ties it to any value that implements
/// [`DeserializeOwned`] and [`Serialize`]
macro_rules! kv_def {
    ($key_id:ident, $val_struct:ident) => {
        #[derive(Clone, Debug)]
        pub struct $key_id(String);

        impl AsRef<str> for $key_id {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl CacheKey for $key_id {}

        impl CacheValue for $val_struct {
            type Key = $key_id;
        }
    };
}
pub(crate) use kv_def;

/// A Redis-based cache of data to avoid expensive fetches from PostgreSQL. Simply a wrapper over
/// Redis.
#[derive(Debug, Clone)]
pub struct RedisCache<M>
where
    M: ManageConnection + Clone,
    M::Connection: ConnectionLike,
{
    redis: Pool<M>,
}

impl<M> RedisCache<M>
where
    M: ManageConnection + Clone,
    M::Connection: ConnectionLike,
{
    pub fn new(redis: Pool<M>) -> RedisCache<M> {
        RedisCache { redis }
    }

    pub async fn get<T: CacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        let mut pool = self
            .redis
            .get()
            .await
            .map_err(|_| Error::Input("FIXME".to_string()))?;
        let fetched = pool.get::<&str, Option<String>>(key.as_ref()).await?;
        Ok(fetched
            .map(|json| serde_json::from_str(&json))
            .transpose()?)
    }

    /// Sets a CacheKey to its associated CacheValue.
    /// Note that the [`Duration`] used is down to millisecond precision.
    pub async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        let mut pool = self
            .redis
            .get()
            .await
            .map_err(|_| Error::Input("FIXME".to_string()))?;

        pool.pset_ex(
            key.as_ref(),
            serde_json::to_string(value)?,
            ttl.as_millis().try_into().map_err(|e| {
                Error::Input(format!(
                    "Duration given cannot be converted to usize: {}",
                    e
                ))
            })?,
        )
        .await?;

        Ok(())
    }

    #[cfg(test)]
    pub async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        let mut pool = self
            .redis
            .get()
            .await
            .map_err(|_| Error::Input("FIXME".to_string()))?;
        pool.del(key.as_ref()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bb8_redis::RedisConnectionManager;
    use serde::Deserialize;

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

    #[tokio::test]
    async fn test_cache_crud_no_ttl() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let redis_pool = bb8::Pool::builder()
            .build(RedisConnectionManager::new(cfg.redis_dsn.as_deref().unwrap()).unwrap())
            .await
            .unwrap();

        let cache = RedisCache::new(redis_pool.clone());

        let (first_key, first_val_a, first_val_b) =
            (TestKeyA::new("1".to_owned()), TestValA(1), TestValA(2));
        let (second_key, second_val_a, second_val_b) = (
            TestKeyB::new("1".to_owned()),
            TestValB("1".to_owned()),
            TestValB("2".to_owned()),
        );

        // Create
        assert!(cache
            .set(&first_key, &first_val_a, Duration::from_secs(30))
            .await
            .is_ok());
        assert!(cache
            .set(&second_key, &second_val_a, Duration::from_secs(30))
            .await
            .is_ok());

        // Read
        assert_eq!(cache.get(&first_key).await.unwrap(), Some(first_val_a));
        assert_eq!(cache.get(&second_key).await.unwrap(), Some(second_val_a));

        // Update (overwrite)
        assert!(cache
            .set(&first_key, &first_val_b, Duration::from_secs(30))
            .await
            .is_ok());
        assert!(cache
            .set(&second_key, &second_val_b, Duration::from_secs(30))
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

        let redis_pool = bb8::Pool::builder()
            .build(RedisConnectionManager::new(cfg.redis_dsn.as_deref().unwrap()).unwrap())
            .await
            .unwrap();
        let cache = RedisCache::new(redis_pool.clone());
        let key = TestKeyA::new("key".to_owned());

        assert!(cache
            .set(&key, &TestValA(1), Duration::from_secs(1))
            .await
            .is_ok());
        tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
        assert_eq!(cache.get::<TestValA>(&key).await.unwrap(), None);
    }
}
