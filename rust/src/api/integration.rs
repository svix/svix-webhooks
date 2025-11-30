// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct IntegrationListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct IntegrationCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct IntegrationRotateKeyOptions {
    pub idempotency_key: Option<String>,
}

pub struct Integration<'a> {
    cfg: &'a Configuration,
}

impl<'a> Integration<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self {
            cfg,
        }
    }

    /// List the application's integrations.
    pub async fn list(
        &self,
        app_id: String,
        options: Option<IntegrationListOptions>,
    ) -> Result<ListResponseIntegrationOut> {
        let IntegrationListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/integration",
        )
            .with_path_param("app_id", app_id)
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    /// Create an integration.
    pub async fn create(
        &self,
        app_id: String,
        integration_in: IntegrationIn,
        options: Option<IntegrationCreateOptions>,
    ) -> Result<IntegrationOut> {
        let IntegrationCreateOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/integration",
        )
            .with_path_param("app_id", app_id)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(integration_in)
            .execute(self.cfg)
            .await
    }

    /// Get an integration.
    pub async fn get(
        &self,
        app_id: String,
        integ_id: String,
    ) -> Result<IntegrationOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/integration/{integ_id}",
        )
            .with_path_param("app_id", app_id)
            .with_path_param("integ_id", integ_id)
            .execute(self.cfg)
            .await
    }

    /// Update an integration.
    pub async fn update(
        &self,
        app_id: String,
        integ_id: String,
        integration_update: IntegrationUpdate,
    ) -> Result<IntegrationOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/app/{app_id}/integration/{integ_id}",
        )
            .with_path_param("app_id", app_id)
            .with_path_param("integ_id", integ_id)
            .with_body_param(integration_update)
            .execute(self.cfg)
            .await
    }

    /// Delete an integration.
    pub async fn delete(
        &self,
        app_id: String,
        integ_id: String,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/app/{app_id}/integration/{integ_id}",
        )
            .with_path_param("app_id", app_id)
            .with_path_param("integ_id", integ_id)
            .returns_nothing()
            .execute(self.cfg)
            .await
    }

    /// Get an integration's key.
    #[deprecated]
    pub async fn get_key(
        &self,
        app_id: String,
        integ_id: String,
    ) -> Result<IntegrationKeyOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/app/{app_id}/integration/{integ_id}/key",
        )
            .with_path_param("app_id", app_id)
            .with_path_param("integ_id", integ_id)
            .execute(self.cfg)
            .await
    }

    /// Rotate the integration's key. The previous key will be immediately revoked.
    pub async fn rotate_key(
        &self,
        app_id: String,
        integ_id: String,
        options: Option<IntegrationRotateKeyOptions>,
    ) -> Result<IntegrationKeyOut> {
        let IntegrationRotateKeyOptions {
            idempotency_key,
        } = options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
        )
            .with_path_param("app_id", app_id)
            .with_path_param("integ_id", integ_id)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .execute(self.cfg)
            .await
    }
}
