// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use enum_dispatch::enum_dispatch;

use crate::redis::RedisPool;

use super::kv_backend::{
    memory::MemoryKeyValueStore, none::NoKeyValueStore, redis::RedisKeyValueStore, Key,
    KeyValueStoreBackend, Result, Value,
};

#[derive(Clone)]
#[enum_dispatch(KeyValueStoreBackend)]
pub enum Cache {
    MemoryCache(MemoryKeyValueStore),
    RedisCache(RedisKeyValueStore),
    None(NoKeyValueStore),
}

impl Cache {
    pub fn new_none() -> Cache {
        Cache::None(NoKeyValueStore)
    }

    pub fn new_memory() -> Cache {
        Cache::MemoryCache(MemoryKeyValueStore::default())
    }

    pub fn new_redis(redis: RedisPool) -> Cache {
        Cache::RedisCache(RedisKeyValueStore::new(redis))
    }

    pub fn is_none(&self) -> bool {
        matches!(*self, Cache::None(_))
    }

    pub async fn get<T: CacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        KeyValueStoreBackend::get(self, key).await
    }

    pub async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        KeyValueStoreBackend::set(self, key, value, ttl).await
    }

    pub async fn set_if_not_exists<T: CacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        KeyValueStoreBackend::set_if_not_exists(self, key, value, ttl).await
    }

    pub async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        KeyValueStoreBackend::delete(self, key).await
    }
}

pub trait CacheKey: Key {}
pub trait CacheValue: Value {}
macro_rules! cache_kv_def {
    ($key_id: ident, $val_id:ident, $prefix:literal) => {
        crate::core::kv_backend::kv_def!($key_id, $val_id, $prefix, "SVIX_CACHE_");

        impl crate::core::cache::CacheKey for $key_id {}
        impl crate::core::cache::CacheValue for $val_id {}
    };
}
pub(crate) use cache_kv_def;
