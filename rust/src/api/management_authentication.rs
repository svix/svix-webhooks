use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct ManagementAuthenticationListApiTokensOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct ManagementAuthenticationCreateApiTokenOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct ManagementAuthenticationExpireApiTokenOptions {
    pub idempotency_key: Option<String>,
}

pub struct ManagementAuthentication<'a> {
    cfg: &'a Configuration,
}

impl<'a> ManagementAuthentication<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List all API Tokens.
    pub async fn list_api_tokens(
        &self,
        options: Option<ManagementAuthenticationListApiTokensOptions>,
    ) -> Result<ListResponseApiTokenCensoredOut> {
        let ManagementAuthenticationListApiTokensOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/management/authentication/api-token",
        )
        .with_optional_query_param("limit", limit)
        .with_optional_query_param("iterator", iterator)
        .with_optional_query_param("order", order)
        .execute(self.cfg)
        .await
    }

    /// Create a new API Token.
    pub async fn create_api_token(
        &self,
        api_token_in: ApiTokenIn,
        options: Option<ManagementAuthenticationCreateApiTokenOptions>,
    ) -> Result<ApiTokenOut> {
        let ManagementAuthenticationCreateApiTokenOptions { idempotency_key } =
            options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/management/authentication/api-token",
        )
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(api_token_in)
        .execute(self.cfg)
        .await
    }

    /// Expire the selected API Token.
    pub async fn expire_api_token(
        &self,
        key_id: String,
        api_token_expire_in: ApiTokenExpireIn,
        options: Option<ManagementAuthenticationExpireApiTokenOptions>,
    ) -> Result<()> {
        let ManagementAuthenticationExpireApiTokenOptions { idempotency_key } =
            options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/management/authentication/api-token/{key_id}/expire",
        )
        .with_path_param("key_id", key_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(api_token_expire_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
