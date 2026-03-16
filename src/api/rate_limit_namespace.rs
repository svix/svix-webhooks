// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct RateLimitNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> RateLimitNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create rate limiter namespace
    pub async fn create(
        &self,
        rate_limit_create_namespace_in: RateLimitCreateNamespaceIn,
    ) -> Result<RateLimitCreateNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limit/namespace/create")
            .with_body(rate_limit_create_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get rate limiter namespace
    pub async fn get(
        &self,
        rate_limit_get_namespace_in: RateLimitGetNamespaceIn,
    ) -> Result<RateLimitGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1/rate-limit/namespace/get")
            .with_body(rate_limit_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
