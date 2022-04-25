// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use axum::async_trait;

use super::{Cache, CacheBehavior, CacheKey, CacheValue, Result};

pub fn new() -> Cache {
    tracing::warn!("Running with caching disabled will negatively affect performance. Idempotency is not supported without a cache.");
    NoCache {}.into()
}

#[derive(Clone)]
pub struct NoCache;

#[async_trait]
impl CacheBehavior for NoCache {
    async fn get<T: CacheValue>(&self, _key: &T::Key) -> Result<Option<T>> {
        Ok(None)
    }

    async fn set<T: CacheValue>(&self, _key: &T::Key, _value: &T, _ttl: Duration) -> Result<()> {
        Ok(())
    }

    async fn delete<T: CacheKey>(&self, _key: &T) -> Result<()> {
        Ok(())
    }

    async fn set_if_not_exists<T: CacheValue>(
        &self,
        _key: &T::Key,
        _value: &T,
        _ttl: Duration,
    ) -> Result<bool> {
        Ok(false)
    }
}
