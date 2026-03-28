// this file is @generated
use crate::{Configuration, error::Result, models::*};

pub struct AdminAuthToken<'a> {
    cfg: &'a Configuration,
}

impl<'a> AdminAuthToken<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Create an auth token
    pub async fn create(
        &self,
        admin_auth_token_create_in: AdminAuthTokenCreateIn,
    ) -> Result<AdminAuthTokenCreateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-token.create")
            .with_body(admin_auth_token_create_in)
            .execute(self.cfg)
            .await
    }

    /// Expire an auth token
    pub async fn expire(
        &self,
        admin_auth_token_expire_in: AdminAuthTokenExpireIn,
    ) -> Result<AdminAuthTokenExpireOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-token.expire")
            .with_body(admin_auth_token_expire_in)
            .execute(self.cfg)
            .await
    }

    /// Rotate an auth token, invalidating the old one and issuing a new secret
    pub async fn rotate(
        &self,
        admin_auth_token_rotate_in: AdminAuthTokenRotateIn,
    ) -> Result<AdminAuthTokenRotateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-token.rotate")
            .with_body(admin_auth_token_rotate_in)
            .execute(self.cfg)
            .await
    }

    /// Delete an auth token
    pub async fn delete(
        &self,
        admin_auth_token_delete_in: AdminAuthTokenDeleteIn,
    ) -> Result<AdminAuthTokenDeleteOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-token.delete")
            .with_body(admin_auth_token_delete_in)
            .execute(self.cfg)
            .await
    }

    /// List auth tokens for a given owner
    pub async fn list(
        &self,
        admin_auth_token_list_in: AdminAuthTokenListIn,
    ) -> Result<ListResponseAdminAuthTokenOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-token.list")
            .with_body(admin_auth_token_list_in)
            .execute(self.cfg)
            .await
    }

    /// Update an auth token's properties
    pub async fn update(
        &self,
        admin_auth_token_update_in: AdminAuthTokenUpdateIn,
    ) -> Result<AdminAuthTokenUpdateOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-token.update")
            .with_body(admin_auth_token_update_in)
            .execute(self.cfg)
            .await
    }

    /// Return the role of the currently authenticated token
    pub async fn whoami(
        &self,
        admin_auth_token_whoami_in: AdminAuthTokenWhoamiIn,
    ) -> Result<AdminAuthTokenWhoamiOut> {
        crate::request::Request::new(http::Method::POST, "/api/v1.admin.auth-token.whoami")
            .with_body(admin_auth_token_whoami_in)
            .execute(self.cfg)
            .await
    }
}
