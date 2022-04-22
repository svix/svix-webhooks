// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use tokio::{
    task,
    time::{sleep, Duration, Instant},
};

use axum::async_trait;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{Cache, CacheBehavior, CacheKey, CacheValue, Result};

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
                .expect("Could not get write lock on memory cache")
                .retain(|_, v| check_is_expired(v))
        }
    });

    MemoryCache { map: shared_state }.into()
}

#[derive(Clone)]
pub struct MemoryCache {
    map: SharedState,
}

#[async_trait]
impl CacheBehavior for MemoryCache {
    async fn get<T: CacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        Ok(self
            .map
            .read()
            .expect("Could not get read lock on memory cache")
            .get(key.as_ref())
            .filter(|wrapper| check_is_expired(wrapper))
            .map(|wrapper| serde_json::from_str(&wrapper.value))
            .transpose()?)
    }

    async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        self.map
            .write()
            .expect("Could not get write lock on memory cache")
            .insert(
                String::from(key.as_ref()),
                ValueWrapper::new(serde_json::to_string(value)?, ttl),
            );

        Ok(())
    }

    async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        self.map
            .write()
            .expect("Could not get write lock on memory cache")
            .remove(key.as_ref());

        Ok(())
    }

    async fn set_if_not_exists<T: CacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        let mut lock = self
            .map
            .write()
            .expect("Could not get write lock on memory cache");

        // TODO: use HashMap::try_insert when stable
        // https://github.com/rust-lang/rust/issues/82766
        if !lock.contains_key(key.as_ref()) {
            lock.insert(
                String::from(key.as_ref()),
                ValueWrapper::new(serde_json::to_string(value)?, ttl),
            );
            return Ok(true);
        }

        Ok(false)
    }
}

fn check_is_expired(vw: &ValueWrapper) -> bool {
    vw.timer.elapsed().as_millis() <= vw.ttl.as_millis()
}

#[cfg(test)]
mod tests {
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
