// this file is @generated
use super::CacheNamespace;
use crate::{Configuration, error::Result, models::*};

pub struct Cache<'a> {
    cfg: &'a Configuration,
}

impl<'a> Cache<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn namespace(&self) -> CacheNamespace<'a> {
        CacheNamespace::new(self.cfg)
    }

    /// Cache Set
    pub async fn set(&self, key: String, cache_set_in: CacheSetIn) -> Result<CacheSetOut> {
        let cache_set_in = CacheSetIn_ {
            namespace: cache_set_in.namespace,
            key,
            value: cache_set_in.value,
            ttl_ms: cache_set_in.ttl_ms,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.cache.set")
            .with_body(cache_set_in)
            .execute(self.cfg)
            .await
    }

    /// Cache Get
    pub async fn get(&self, key: String, cache_get_in: CacheGetIn) -> Result<CacheGetOut> {
        let cache_get_in = CacheGetIn_ {
            namespace: cache_get_in.namespace,
            key,
            consistency: cache_get_in.consistency,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.cache.get")
            .with_body(cache_get_in)
            .execute(self.cfg)
            .await
    }

    /// Cache Delete
    pub async fn delete(
        &self,
        key: String,
        cache_delete_in: CacheDeleteIn,
    ) -> Result<CacheDeleteOut> {
        let cache_delete_in = CacheDeleteIn_ {
            namespace: cache_delete_in.namespace,
            key,
        };

        crate::request::Request::new(http::Method::POST, "/api/v1.cache.delete")
            .with_body(cache_delete_in)
            .execute(self.cfg)
            .await
    }
}
