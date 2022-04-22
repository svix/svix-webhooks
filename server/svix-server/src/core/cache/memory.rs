// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use tokio::{
    task,
    time::{sleep, Duration, Instant},
};

use axum::async_trait;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

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
type SharedState = Arc<Mutex<State>>;

pub fn new() -> Cache {
    let shared_state = Arc::new(Mutex::new(State::new()));

    let shared_state_clone = shared_state.clone();
    task::spawn(async move {
        loop {
            sleep(Duration::from_secs(30)).await;
            remove_expired_keys(&shared_state_clone);
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
        let value = get_mutex_lock(&self.map)
            .get(key.as_ref())
            .filter(|wrapper| check_is_expired(&wrapper))
            .map(|wrapper| serde_json::from_str(&wrapper.value))
            .transpose()?;

        Ok(value)
    }

    async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        get_mutex_lock(&self.map).insert(
            String::from(key.as_ref()),
            ValueWrapper::new(serde_json::to_string(value)?, ttl),
        );

        Ok(())
    }

    async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        get_mutex_lock(&self.map).remove(key.as_ref());

        Ok(())
    }

    async fn set_if_not_exists<T: CacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        let mut lock = get_mutex_lock(&self.map);

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

fn get_mutex_lock(map: &SharedState) -> MutexGuard<State> {
    map.lock().expect("Could not lock on memory cache")
}

fn remove_expired_keys(map: &SharedState) {
    let mut map = get_mutex_lock(&map);
    map.retain(|_, v| check_is_expired(&v))
}

fn check_is_expired(vw: &ValueWrapper) -> bool {
    vw.timer.elapsed().as_millis() <= vw.ttl.as_millis()
}
