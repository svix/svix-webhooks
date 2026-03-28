// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct AuthTokenNamespace<'a> {
    cfg: &'a Configuration,
}

impl<'a> AuthTokenNamespace<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create Auth Token namespace
    pub async fn create(
        &self,
        auth_token_create_namespace_in: AuthTokenCreateNamespaceIn,
    ) -> Result<AuthTokenCreateNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.namespace.create")
            .with_body(auth_token_create_namespace_in)
            .execute(self.cfg)
            .await
    }

    /// Get Auth Token namespace
    pub async fn get(
        &self,
        auth_token_get_namespace_in: AuthTokenGetNamespaceIn,
    ) -> Result<AuthTokenGetNamespaceOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.namespace.get")
            .with_body(auth_token_get_namespace_in)
            .execute(self.cfg)
            .await
    }
}
