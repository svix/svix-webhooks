// this file is @generated
use super::AuthTokenNamespace;
use crate::{Configuration, error::Result, models::*};

pub struct AuthToken<'a> {
    cfg: &'a Configuration,
}

impl<'a> AuthToken<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub fn namespace(&self) -> AuthTokenNamespace<'a> {
        AuthTokenNamespace::new(self.cfg)
    }

    /// Create Auth Token
    pub async fn create(
        &self,
        auth_token_create_in: AuthTokenCreateIn,
    ) -> Result<AuthTokenCreateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.create")
            .with_body(auth_token_create_in)
            .execute(self.cfg)
            .await
    }

    /// Expire Auth Token
    pub async fn expire(
        &self,
        auth_token_expire_in: AuthTokenExpireIn,
    ) -> Result<AuthTokenExpireOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.expire")
            .with_body(auth_token_expire_in)
            .execute(self.cfg)
            .await
    }

    /// Delete Auth Token
    pub async fn delete(
        &self,
        auth_token_delete_in: AuthTokenDeleteIn,
    ) -> Result<AuthTokenDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.delete")
            .with_body(auth_token_delete_in)
            .execute(self.cfg)
            .await
    }

    /// Verify Auth Token
    pub async fn verify(
        &self,
        auth_token_verify_in: AuthTokenVerifyIn,
    ) -> Result<AuthTokenVerifyOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.verify")
            .with_body(auth_token_verify_in)
            .execute(self.cfg)
            .await
    }

    /// List Auth Tokens
    pub async fn list(
        &self,
        auth_token_list_in: AuthTokenListIn,
    ) -> Result<ListResponseAuthTokenOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.list")
            .with_body(auth_token_list_in)
            .execute(self.cfg)
            .await
    }

    /// Update Auth Token
    pub async fn update(
        &self,
        auth_token_update_in: AuthTokenUpdateIn,
    ) -> Result<AuthTokenUpdateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.update")
            .with_body(auth_token_update_in)
            .execute(self.cfg)
            .await
    }

    /// Rotate Auth Token
    pub async fn rotate(
        &self,
        auth_token_rotate_in: AuthTokenRotateIn,
    ) -> Result<AuthTokenRotateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.auth-token.rotate")
            .with_body(auth_token_rotate_in)
            .execute(self.cfg)
            .await
    }
}
