// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct IdempotencyNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> IdempotencyNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create idempotency namespace
    pub async fn create(
        &self,
        idempotency_create_namespace_in: IdempotencyCreateNamespaceIn,
    ) -> Result<IdempotencyCreateNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.idempotency.namespace.create")
            .with_body(idempotency_create_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get idempotency namespace
    pub async fn get(
        &self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> Result<IdempotencyGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.idempotency.namespace.get")
            .with_body(idempotency_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
