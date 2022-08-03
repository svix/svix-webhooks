// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{string::FromUtf8Error, time::Duration};

use ::redis::RedisError;
use axum::async_trait;
use enum_dispatch::enum_dispatch;
use serde::{de::DeserializeOwned, Serialize};

use super::cache::Cache;
use crate::core::run_with_retries::run_with_retries;

pub mod memory;
use memory::MemoryKeyValueStore;

pub mod none;
use none::NoKeyValueStore;

pub mod redis;
use self::redis::RedisKeyValueStore;

/// Errors internal to the KV Backends
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
pub type Result<T> = std::result::Result<T, Error>;

/// A valid key value for the shared_store -- usually just a wrapper around a [`String`]
pub trait Key: AsRef<str> + Send + Sync {
    const KEY_PREFIX: &'static str;
    const KV_STORE_TY: &'static str;
}
/// Any (de)serializable structure usuable as a value in the shared_store or cache -- it is
/// associated with a given key type to ensure type checking on creation or reading of values from
///  the shared_store
pub trait Value: DeserializeOwned + Serialize + Send + Sync {
    type Key: Key;
}

pub trait StringValue: ToString + TryFrom<String> + Send + Sync {
    type Key: Key;
}

/// A macro that creates a [`Key`] and ties it to any value that implements [`DeserializeOwned`] and
/// [`Serialize`]
macro_rules! kv_def {
    ($key_id:ident, $val_struct:ident, $lit_prefix:literal, $store_ty_prefix:literal) => {
        #[derive(Clone, Debug)]
        pub struct $key_id(String);

        impl AsRef<str> for $key_id {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl crate::core::kv_backend::Value for $val_struct {
            type Key = $key_id;
        }

        impl crate::core::kv_backend::Key for $key_id {
            const KEY_PREFIX: &'static str = $lit_prefix;
            const KV_STORE_TY: &'static str = $store_ty_prefix;
        }
    };
}
pub(crate) use kv_def;

// Used downstream and for testing:
#[allow(unused_macros)]
macro_rules! string_kv_def {
    ($key_id:ident, $val_struct:ident, $lit_prefix:literal, $store_ty_prefix:literal) => {
        #[derive(Clone, Debug)]
        pub struct $key_id(String);

        impl AsRef<str> for $key_id {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl crate::core::kv_backend::StringValue for $val_struct {
            type Key = $key_id;
        }

        impl crate::core::kv_backend::Key for $key_id {
            const KEY_PREFIX: &'static str = $lit_prefix;
            const KV_STORE_TY: &'static str = $store_ty_prefix;
        }
    };
}
#[allow(unused_imports)]
pub(crate) use string_kv_def;

const RETRY_SCHEDULE: &[Duration] = &[
    Duration::from_millis(10),
    Duration::from_millis(20),
    Duration::from_millis(40),
];

fn prefixed_key<T: Key>(key: &T) -> Vec<u8> {
    let prefixed = format!("{}{}{}", T::KV_STORE_TY, T::KEY_PREFIX, key.as_ref());
    prefixed.into()
}

#[async_trait]
#[enum_dispatch]
pub trait KeyValueStoreBackend: Sync + Send {
    fn should_retry(&self, e: &crate::core::kv_backend::Error) -> bool;

    // NOTE: The fully expanded types (`std::result::Result<_, crate::core::kv_backend::Error>`) are
    // because the `enum_dispatch` crate expands based on these defintions. So they have to be expanded
    // for the macro to be sanitized.

    async fn get<T: Value>(
        &self,
        key: &T::Key,
    ) -> std::result::Result<Option<T>, crate::core::kv_backend::Error> {
        run_with_retries(
            || async move {
                self.get_raw(&prefixed_key(key))
                    .await?
                    .map(|x| {
                        String::from_utf8(x)
                            .map_err(|e| e.into())
                            .and_then(|json| serde_json::from_str(&json).map_err(|e| e.into()))
                    })
                    .transpose()
            },
            |e| self.should_retry(e),
            RETRY_SCHEDULE,
        )
        .await
    }

    async fn get_raw(
        &self,
        key: &[u8],
    ) -> std::result::Result<Option<Vec<u8>>, crate::core::kv_backend::Error>;

    async fn get_string<T: StringValue>(
        &self,
        key: &T::Key,
    ) -> std::result::Result<Option<T>, crate::core::kv_backend::Error> {
        run_with_retries(
            || async move {
                self.get_raw(&prefixed_key(key))
                    .await?
                    .map(|x| {
                        String::from_utf8(x)
                            .map_err(|e| e.into())
                            .and_then(|x| x.try_into().map_err(|_| Error::DeserializationOther))
                    })
                    .transpose()
            },
            |e| self.should_retry(e),
            RETRY_SCHEDULE,
        )
        .await
    }

    async fn set<T: Value>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> std::result::Result<(), crate::core::kv_backend::Error> {
        run_with_retries(
            || async move {
                self.set_raw(
                    &prefixed_key(key),
                    serde_json::to_string(value)?.as_bytes(),
                    ttl,
                )
                .await
            },
            |e| self.should_retry(e),
            RETRY_SCHEDULE,
        )
        .await
    }

    async fn set_raw(
        &self,
        key: &[u8],
        value: &[u8],
        ttl: Duration,
    ) -> std::result::Result<(), crate::core::kv_backend::Error>;

    async fn set_string<T: StringValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> std::result::Result<(), crate::core::kv_backend::Error> {
        run_with_retries(
            || async move {
                self.set_raw(&prefixed_key(key), value.to_string().as_bytes(), ttl)
                    .await
            },
            |e| self.should_retry(e),
            RETRY_SCHEDULE,
        )
        .await
    }

    async fn delete<T: Key>(
        &self,
        key: &T,
    ) -> std::result::Result<(), crate::core::kv_backend::Error> {
        self.delete_raw(&prefixed_key(key)).await
    }

    async fn delete_raw(
        &self,
        key: &[u8],
    ) -> std::result::Result<(), crate::core::kv_backend::Error>;

    async fn set_if_not_exists<T: Value>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> std::result::Result<bool, crate::core::kv_backend::Error> {
        run_with_retries(
            || async move {
                self.set_raw_if_not_exists(
                    &prefixed_key(key),
                    serde_json::to_string(value)?.as_bytes(),
                    ttl,
                )
                .await
            },
            |e| self.should_retry(e),
            RETRY_SCHEDULE,
        )
        .await
    }

    async fn set_raw_if_not_exists(
        &self,
        key: &[u8],
        value: &[u8],
        ttl: Duration,
    ) -> std::result::Result<bool, crate::core::kv_backend::Error>;

    async fn set_string_if_not_exists<T: StringValue>(
        &self,
        key: &T::Key,
        value: &T,
        ttl: Duration,
    ) -> std::result::Result<bool, crate::core::kv_backend::Error> {
        run_with_retries(
            || async move {
                self.set_raw_if_not_exists(&prefixed_key(key), value.to_string().as_bytes(), ttl)
                    .await
            },
            |e| self.should_retry(e),
            RETRY_SCHEDULE,
        )
        .await
    }
}
