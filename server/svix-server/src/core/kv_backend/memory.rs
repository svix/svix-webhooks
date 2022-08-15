// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use tokio::{
    sync::RwLock,
    task,
    time::{sleep, Duration, Instant},
};

use axum::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use super::{KeyValueStoreBackend, Result};

#[derive(Debug)]
struct ValueWrapper {
    value: Vec<u8>,
    ttl: Duration,
    timer: Instant,
}

impl ValueWrapper {
    fn new(value: Vec<u8>, ttl: Duration) -> ValueWrapper {
        ValueWrapper {
            value,
            ttl,
            timer: Instant::now(),
        }
    }
}

type State = HashMap<Vec<u8>, ValueWrapper>;
type SharedState = Arc<RwLock<State>>;

#[derive(Clone)]
pub struct MemoryKeyValueStore {
    map: SharedState,
}

impl MemoryKeyValueStore {
    pub fn new() -> MemoryKeyValueStore {
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

        MemoryKeyValueStore { map: shared_state }
    }
}

impl Default for MemoryKeyValueStore {
    fn default() -> Self {
        MemoryKeyValueStore::new()
    }
}

#[async_trait]
impl KeyValueStoreBackend for MemoryKeyValueStore {
    fn should_retry(&self, _e: &super::Error) -> bool {
        false
    }

    async fn get_raw(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self
            .map
            .read()
            .await
            .get(key)
            .filter(|wrapper| check_is_expired(wrapper))
            .map(|wrapper| wrapper.value.clone()))
    }

    async fn set_raw(&self, key: &[u8], value: &[u8], ttl: Duration) -> Result<()> {
        self.map
            .write()
            .await
            .insert(key.to_owned(), ValueWrapper::new(value.to_owned(), ttl));
        Ok(())
    }

    async fn set_raw_if_not_exists(&self, key: &[u8], value: &[u8], ttl: Duration) -> Result<bool> {
        let mut lock = self.map.write().await;

        // TODO: use HashMap::try_insert when stable
        // https://github.com/rust-lang/rust/issues/82766
        if !lock.contains_key(key) {
            lock.insert(key.to_owned(), ValueWrapper::new(value.to_owned(), ttl));
            return Ok(true);
        }

        Ok(false)
    }

    async fn delete_raw(&self, key: &[u8]) -> Result<()> {
        self.map.write().await.remove(key);

        Ok(())
    }
}

fn check_is_expired(vw: &ValueWrapper) -> bool {
    vw.timer.elapsed().as_millis() <= vw.ttl.as_millis()
}

#[cfg(test)]
mod tests {
    use super::{
        super::{kv_def, string_kv_def},
        *,
    };
    use serde::{Deserialize, Serialize};

    // Test structures

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestValA(usize);
    kv_def!(TestKeyA, TestValA, "SVIX_TEST_KEY_A_", "TEST_");
    impl TestKeyA {
        fn new(id: String) -> TestKeyA {
            TestKeyA(id)
        }
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestValB(String);
    kv_def!(TestKeyB, TestValB, "SVIX_TEST_KEY_B_", "TEST_");
    impl TestKeyB {
        fn new(id: String) -> TestKeyB {
            TestKeyB(id)
        }
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct StringTestVal(String);
    string_kv_def!(
        StringTestKey,
        StringTestVal,
        "SVIX_TEST_KEY_STRING_",
        "TEST_"
    );
    impl StringTestKey {
        fn new(id: String) -> StringTestKey {
            StringTestKey(id)
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
    async fn test_kv_crud_no_ttl() {
        let kv = MemoryKeyValueStore::new();

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
        assert!(kv
            .set(&first_key, &first_val_a, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(kv
            .set(&second_key, &second_val_a, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(kv
            .set_string(&third_key, &third_val_a, Duration::from_secs(30),)
            .await
            .is_ok());

        // Read
        assert_eq!(kv.get(&first_key).await.unwrap(), Some(first_val_a));
        assert_eq!(kv.get(&second_key).await.unwrap(), Some(second_val_a));
        assert_eq!(kv.get_string(&third_key).await.unwrap(), Some(third_val_a));

        // Update (overwrite)
        assert!(kv
            .set(&first_key, &first_val_b, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(kv
            .set(&second_key, &second_val_b, Duration::from_secs(30),)
            .await
            .is_ok());
        assert!(kv
            .set_string(&third_key, &third_val_b, Duration::from_secs(30),)
            .await
            .is_ok());

        // Confirm update
        assert_eq!(kv.get(&first_key).await.unwrap(), Some(first_val_b));
        assert_eq!(kv.get(&second_key).await.unwrap(), Some(second_val_b));
        assert_eq!(kv.get_string(&third_key).await.unwrap(), Some(third_val_b));

        // Delete
        assert!(kv.delete(&first_key).await.is_ok());
        assert!(kv.delete(&second_key).await.is_ok());
        assert!(kv.delete(&third_key).await.is_ok());

        // Confirm deletion
        assert_eq!(kv.get::<TestValA>(&first_key).await.unwrap(), None);
        assert_eq!(kv.get::<TestValB>(&second_key).await.unwrap(), None);
        assert_eq!(
            kv.get_string::<StringTestVal>(&third_key).await.unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn test_kv_ttl() {
        let kv = MemoryKeyValueStore::new();
        let key = TestKeyA::new("key".to_owned());

        assert!(kv
            .set(&key, &TestValA(1), Duration::from_secs(1),)
            .await
            .is_ok());
        tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
        assert_eq!(kv.get::<TestValA>(&key).await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_kv_nx_status() {
        let kv = MemoryKeyValueStore::new();
        let key = TestKeyA::new("nx_status_test_key".to_owned());

        assert!(kv
            .set_if_not_exists(&key, &TestValA(1), Duration::from_secs(30),)
            .await
            .unwrap());
        assert_eq!(kv.get(&key).await.unwrap(), Some(TestValA(1)));

        assert!(!kv
            .set_if_not_exists(&key, &TestValA(2), Duration::from_secs(30),)
            .await
            .unwrap());
        assert_eq!(kv.get(&key).await.unwrap(), Some(TestValA(1)));

        assert!(kv.delete(&key).await.is_ok());
    }
}
