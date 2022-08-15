// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use axum::async_trait;

use super::{KeyValueStoreBackend, Result, StringValue, Value};

#[derive(Clone)]
pub struct NoKeyValueStore;

#[async_trait]
impl KeyValueStoreBackend for NoKeyValueStore {
    fn should_retry(&self, _e: &super::Error) -> bool {
        false
    }

    async fn get<T: Value>(&self, _key: &T::Key) -> Result<Option<T>> {
        Ok(None)
    }

    async fn get_raw(&self, _key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    async fn get_string<T: StringValue>(&self, _key: &T::Key) -> Result<Option<T>> {
        Ok(None)
    }

    async fn set<T: Value>(&self, _key: &T::Key, _value: &T, _ttl: Duration) -> Result<()> {
        Ok(())
    }

    async fn set_raw(&self, _key: &[u8], _value: &[u8], _ttl: Duration) -> Result<()> {
        Ok(())
    }

    async fn set_string<T: StringValue>(
        &self,
        _key: &T::Key,
        _value: &T,
        _ttl: Duration,
    ) -> Result<()> {
        Ok(())
    }

    async fn delete_raw(&self, _key: &[u8]) -> Result<()> {
        Ok(())
    }

    async fn set_if_not_exists<T: Value>(
        &self,
        _key: &T::Key,
        _value: &T,
        _ttl: Duration,
    ) -> Result<bool> {
        Ok(false)
    }

    async fn set_raw_if_not_exists(
        &self,
        _key: &[u8],
        _value: &[u8],
        _ttl: Duration,
    ) -> Result<bool> {
        Ok(false)
    }

    async fn set_string_if_not_exists<T: StringValue>(
        &self,
        _key: &T::Key,
        _value: &T,
        _ttl: Duration,
    ) -> Result<bool> {
        Ok(false)
    }
}
