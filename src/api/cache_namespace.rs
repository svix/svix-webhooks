// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct CacheNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> CacheNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create cache namespace
    pub async fn create(
        &self,
        cache_create_namespace_in: CacheCreateNamespaceIn,
    ) -> Result<CacheCreateNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.cache.namespace.create")
            .with_body(cache_create_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get cache namespace
    pub async fn get(
        &self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> Result<CacheGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.cache.namespace.get")
            .with_body(cache_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
