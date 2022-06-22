// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{string::FromUtf8Error, time::Duration};

use ::redis::RedisError;
use axum::async_trait;
use enum_dispatch::enum_dispatch;
use serde::{de::DeserializeOwned, Serialize};

pub mod memory;
pub mod none;
pub mod redis;

/// Errors internal to the cache
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error deserializing Redis value")]
    Deserialization(#[from] serde_json::error::Error),

    #[error("error deserializing Redis value")]
    DeserializationOther,

    #[error("error deserializing byte array")]
    DeserializationBytes(#[from] FromUtf8Error),

    #[error("Redis pool error")]
    Pool(#[from] bb8::RunError<RedisError>),

    #[error("Redis databse error")]
    Database(#[from] RedisError),

    #[error("input error: {0}")]
    Input(String),
}
type Result<T> = std::result::Result<T, Error>;

/// A valid key value for the cache -- usually just a wrapper around a [`String`]
pub trait CacheKey: AsRef<str> + Send + Sync {
    const PREFIX_CACHE: &'static str = "SVIX_CACHE";
}
/// Any (de)serializable structure usuable as a value in the cache -- it is associated with a
/// given key type to ensure type checking on creation or reading of values from the cache
pub trait CacheValue: DeserializeOwned + Serialize + Send + Sync {
    type Key: CacheKey;
}

pub trait StringCacheValue: ToString + TryFrom<String> + Send + Sync {
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

// Used downstream and for testing:
#[allow(unused_macros)]
macro_rules! string_kv_def {
    ($key_id:ident, $val_struct:ident) => {
        #[derive(Clone, Debug)]
        pub struct $key_id(String);

        impl AsRef<str> for $key_id {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl CacheKey for $key_id {}

        impl StringCacheValue for $val_struct {
            type Key = $key_id;
        }
    };
}
#[allow(unused_imports)]
pub(crate) use string_kv_def;

#[derive(Clone)]
#[enum_dispatch]
pub enum Cache {
    MemoryCache(memory::MemoryCache),
    RedisCache(redis::RedisCache),
    None(none::NoCache),
}

impl Cache {
    pub fn is_none(&self) -> bool {
        matches!(*self, Cache::None(none::NoCache))
    }
}

#[async_trait]
#[enum_dispatch(Cache)]
pub trait CacheBehavior: Sync + Send {
    async fn get<T: CacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        self.get_raw(key.as_ref().as_bytes())
            .await?
            .map(|x| {
                String::from_utf8(x)
                    .map_err(|e| e.into())
                    .and_then(|json| serde_json::from_str(&json).map_err(|e| e.into()))
            })
            .transpose()
    }

    async fn get_raw(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    async fn get_string<T: StringCacheValue>(&self, key: &T::Key) -> Result<Option<T>> {
        self.get_raw(key.as_ref().as_bytes())
            .await?
            .map(|x| {
                String::from_utf8(x)
                    .map_err(|e| e.into())
                    .and_then(|x| x.try_into().map_err(|_| Error::DeserializationOther))
            })
            .transpose()
    }

    async fn set<T: CacheValue>(&self, key: &T::Key, value: &T, ttl: Duration) -> Result<()> {
        self.set_raw(
            key.as_ref().as_bytes(),
            serde_json::to_string(value)?.as_bytes(),
            ttl,
        )
        .await
    }

    async fn set_raw(&self, key: &[u8], value: &[u8], ttl: Duration) -> Result<()>;

    async fn set_string<T: StringCacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<()> {
        self.set_raw(key.as_ref().as_bytes(), value.to_string().as_bytes(), ttl)
            .await
    }

    async fn delete<T: CacheKey>(&self, key: &T) -> Result<()>;

    async fn set_if_not_exists<T: CacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        self.set_raw_if_not_exists(
            key.as_ref().as_bytes(),
            serde_json::to_string(value)?.as_bytes(),
            ttl,
        )
        .await
    }

    async fn set_raw_if_not_exists(&self, key: &[u8], value: &[u8], ttl: Duration) -> Result<bool>;

    async fn set_string_if_not_exists<T: StringCacheValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> Result<bool> {
        self.set_raw_if_not_exists(key.as_ref().as_bytes(), value.to_string().as_bytes(), ttl)
            .await
    }
}
