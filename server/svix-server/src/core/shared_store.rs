// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use enum_dispatch::enum_dispatch;

use crate::redis::RedisPool;

use super::kv_backend::{
    memory::MemoryKeyValueStore, redis::RedisKeyValueStore, Key, KeyValueStoreBackend, Result,
    StringValue, Value,
};

#[derive(Clone)]
#[enum_dispatch(KeyValueStoreBackend)]
pub enum SharedStore {
    MemorySharedStore(MemoryKeyValueStore),
    RedisSharedStore(RedisKeyValueStore),
}

impl SharedStore {
    pub fn new_memory() -> SharedStore {
        SharedStore::MemorySharedStore(MemoryKeyValueStore::default())
    }

    pub fn new_redis(redis: RedisPool) -> SharedStore {
        SharedStore::RedisSharedStore(RedisKeyValueStore::new(redis))
    }

    pub async fn get<T: StoreValue>(&self, key: &T::Key) -> Result<Option<T>> {
        KeyValueStoreBackend::get(self, key).await
    }

    pub async fn set<T: StoreValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        KeyValueStoreBackend::set(self, key, value, ttl).await
    }

    pub async fn set_if_not_exists<T: StoreValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        KeyValueStoreBackend::set_if_not_exists(self, key, value, ttl).await
    }

    pub async fn delete<T: StoreKey>(&self, key: &T) -> Result<()> {
        KeyValueStoreBackend::delete(self, key).await
    }
}

pub trait StoreKey: Key {}
pub trait StoreValue: Value {}
macro_rules! store_kv_def {
    ($key_id: ident, $val_id:ident, $prefix:literal) => {
        crate::core::kv_backend::kv_def!($key_id, $val_id, $prefix, "SVIX_STORE_");

        impl crate::core::shared_store::StoreKey for $key_id {}
        impl crate::core::shared_store::StoreValue for $val_id {}
    };
}
pub(crate) use store_kv_def;
