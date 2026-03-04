// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct KvNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> KvNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create KV namespace
    pub async fn create(
        &self,
        kv_create_namespace_in: KvCreateNamespaceIn,
    ) -> Result<KvCreateNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/kv/namespace/create")
            .with_body(kv_create_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get KV namespace
    pub async fn get(&self, kv_get_namespace_in: KvGetNamespaceIn) -> Result<KvGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/kv/namespace/get")
            .with_body(kv_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
