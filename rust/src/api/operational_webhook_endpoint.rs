use super::{Ordering, PostOptions};
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct OperationalWebhookEndpointListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub order: Option<Ordering>,
}

pub struct OperationalWebhookEndpoint<'a> {
    cfg: &'a Configuration,
}

impl<'a> OperationalWebhookEndpoint<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        options: Option<OperationalWebhookEndpointListOptions>,
    ) -> Result<ListResponseOperationalWebhookEndpointOut> {
        let OperationalWebhookEndpointListOptions {
            iterator,
            limit,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/operational-webhook/endpoint")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    pub async fn create(
        &self,
        endpoint_in: OperationalWebhookEndpointIn,
        options: Option<PostOptions>,
    ) -> Result<OperationalWebhookEndpointOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/api/v1/operational-webhook/endpoint")
            .with_body_param(endpoint_in)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .execute(self.cfg)
            .await
    }

    pub async fn get(&self, endpoint_id: String) -> Result<OperationalWebhookEndpointOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .execute(self.cfg)
        .await
    }

    pub async fn update(
        &self,
        endpoint_id: String,
        endpoint_update: OperationalWebhookEndpointUpdate,
    ) -> Result<OperationalWebhookEndpointOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(endpoint_update)
        .execute(self.cfg)
        .await
    }

    pub async fn delete(&self, endpoint_id: String) -> Result<()> {
        crate::request::Request::new(
            http1::Method::DELETE,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    pub async fn get_secret(
        &self,
        endpoint_id: String,
    ) -> Result<OperationalWebhookEndpointSecretOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .execute(self.cfg)
        .await
    }

    pub async fn rotate_secret(
        &self,
        endpoint_id: String,
        endpoint_secret_rotate_in: OperationalWebhookEndpointSecretIn,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(endpoint_secret_rotate_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
