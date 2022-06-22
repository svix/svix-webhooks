// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use tokio::{
    sync::RwLock,
    task,
    time::{sleep, Duration, Instant},
};

use axum::async_trait;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;

use super::{Cache, CacheBehavior, CacheKey, CacheValue, Error, Result, StringCacheValue};

#[derive(Debug)]
struct ValueWrapper {
    value: String,
    ttl: Duration,
    timer: Instant,
}

impl ValueWrapper {
    fn new(value: String, ttl: Duration) -> ValueWrapper {
        ValueWrapper {
            value,
            ttl,
            timer: Instant::now(),
        }
    }
}

type State = HashMap<String, ValueWrapper>;
type SharedState = Arc<RwLock<State>>;

pub fn new() -> Cache {
    let shared_state = Arc::new(RwLock::new(State::new()));

    let shared_state_clone = shared_state.clone();
    task::spawn(async move {
        loop {
            sleep(Duration::from_secs(60 * 5)).await;
            shared_state_clone
                .write()
                .await
                .retain(|_, v| check_is_expired(v))
        }
    });

    MemoryCache { map: shared_state }.into()
}

#[derive(Clone)]
pub struct MemoryCache {
    map: SharedState,
}

impl MemoryCache {
    async fn get_raw(&self, key: &str) -> Option<String> {
        self.map
            .read()
            .await
            .get(key)
            .filter(|wrapper| check_is_expired(wrapper))
            .map(|wrapper| wrapper.value.clone())
    }

    async fn set_raw(&self, key: &str, value: &str, ttl: Duration) -> Result<()> {
        self.map
            .write()
            .await
            .insert(String::from(key), ValueWrapper::new(value.to_string(), ttl));
        Ok(())
    }

    async fn set_raw_if_not_exists(&self, key: &str, value: &str, ttl: Duration) -> Result<bool> {
        let mut lock = self.map.write().await;

        // TODO: use HashMap::try_insert when stable
        // https://github.com/rust-lang/rust/issues/82766
        if !lock.contains_key(key) {
            lock.insert(key.to_owned(), ValueWrapper::new(value.to_owned(), ttl));
            return Ok(true);
        }

        Ok(false)
    }
}

#[async_trait]
impl CacheBehavior for MemoryCache {
    async fn get<T: CacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        self.get_raw(key.as_ref())
            .await
            .map(|x| serde_json::from_str(&x).map_err(|e| e.into()))
            .transpose()
    }

    async fn get_string<T: StringCacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        self.get_raw(key.as_ref())
            .await
            .map(|x| x.try_into().map_err(|_| Error::DeserializationOther))
            .transpose()
    }

    async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        self.map.write().await.insert(
            String::from(key.as_ref()),
            ValueWrapper::new(serde_json::to_string(value)?, ttl),
        );

        Ok(())
    }

    async fn set_string<T: StringCacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<()> {
        self.set_raw(key.as_ref(), &value.to_string(), ttl).await
    }

    async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        self.map.write().await.remove(key.as_ref());

        Ok(())
    }

    async fn set_if_not_exists<T: CacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        self.set_raw_if_not_exists(key.as_ref(), &serde_json::to_string(value)?, ttl)
            .await
    }

    async fn set_string_if_not_exists<T: StringCacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        self.set_raw_if_not_exists(key.as_ref(), &value.to_string(), ttl)
            .await
    }
}

fn check_is_expired(vw: &ValueWrapper) -> bool {
    vw.timer.elapsed().as_millis() <= vw.ttl.as_millis()
}

#[cfg(test)]
mod tests {
    use crate::core::cache::string_kv_def;

    use super::{super::kv_def, *};
    use serde::{Deserialize, Serialize};

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

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct StringTestVal(String);
    string_kv_def!(StringTestKey, StringTestVal);
    impl StringTestKey {
        fn new(id: String) -> StringTestKey {
            StringTestKey(format!("SVIX_TEST_KEY_STRING_{}", id))
        }
    }

    impl std::fmt::Display for StringTestVal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl TryFrom<String> for StringTestVal {
        type Error = crate::error::Error;
        fn try_from(s: String) -> crate::error::Result<Self> {
            Ok(StringTestVal(s))
        }
    }

    #[tokio::test]
    async fn test_cache_crud_no_ttl() {
        let cache = new();

        let (first_key, first_val_a, first_val_b) =
            (TestKeyA::new("1".to_owned()), TestValA(1), TestValA(2));
        let (second_key, second_val_a, second_val_b) = (
            TestKeyB::new("1".to_owned()),
            TestValB("1".to_owned()),
            TestValB("2".to_owned()),
        );
        let (third_key, third_val_a, third_val_b) = (
            StringTestKey::new("1".to_owned()),
            StringTestVal("1".to_owned()),
            StringTestVal("2".to_owned()),
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
        assert_eq!(
            cache.get_string::<StringTestVal>(&third_key).await.unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_cache_ttl() {
        let cache = new();
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
        let cache = new();
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
