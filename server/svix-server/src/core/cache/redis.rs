// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use axum::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use serde_json;

use super::{Cache, CacheBehavior, CacheKey, CacheValue, Error, Result};

pub fn new(redis: Pool<RedisConnectionManager>) -> Cache {
    RedisCache { redis }.into()
}

#[derive(Clone)]
pub struct RedisCache {
    redis: Pool<RedisConnectionManager>,
}

#[async_trait]
impl CacheBehavior for RedisCache {
    async fn get<T: CacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        let mut pool = self.redis.get().await.unwrap();
        let fetched = pool.get::<&str, Option<String>>(key.as_ref()).await?;

        Ok(fetched
            .map(|json| serde_json::from_str(&json))
            .transpose()?)
    }

    async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        let mut pool = self.redis.get().await?;

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
        .await
        .map_err(Into::into)
    }

    async fn delete<T: CacheKey>(&self, key: &T) -> Result<()> {
        let mut pool = self.redis.get().await?;
        pool.del(key.as_ref()).await?;

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

        let res: Option<()> = cmd.query_async(&mut *pool).await?;

        Ok(res.is_some())
    }
}
