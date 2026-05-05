// this file is @generated
use crate::{error::Result, models::*, Configuration};

#[derive(Default)]
pub struct OperationalWebhookEndpointListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct OperationalWebhookEndpointCreateOptions {
    pub idempotency_key: Option<String>,
}

#[derive(Default)]
pub struct OperationalWebhookEndpointRotateSecretOptions {
    pub idempotency_key: Option<String>,
}

pub struct OperationalWebhookEndpoint<'a> {
    cfg: &'a Configuration,
}

impl<'a> OperationalWebhookEndpoint<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List operational webhook endpoints.
    pub async fn list(
        &self,
        options: Option<OperationalWebhookEndpointListOptions>,
    ) -> Result<ListResponseOperationalWebhookEndpointOut> {
        let OperationalWebhookEndpointListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::GET, "/api/v1/operational-webhook/endpoint")
            .with_optional_query_param("limit", limit)
            .with_optional_query_param("iterator", iterator)
            .with_optional_query_param("order", order)
            .execute(self.cfg)
            .await
    }

    /// Create an operational webhook endpoint.
    pub async fn create(
        &self,
        operational_webhook_endpoint_in: OperationalWebhookEndpointIn,
        options: Option<OperationalWebhookEndpointCreateOptions>,
    ) -> Result<OperationalWebhookEndpointOut> {
        let OperationalWebhookEndpointCreateOptions { idempotency_key } =
            options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/api/v1/operational-webhook/endpoint")
            .with_optional_header_param("idempotency-key", idempotency_key)
            .with_body_param(operational_webhook_endpoint_in)
            .execute(self.cfg)
            .await
    }

    /// Get an operational webhook endpoint.
    pub async fn get(&self, endpoint_id: String) -> Result<OperationalWebhookEndpointOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .execute(self.cfg)
        .await
    }

    /// Update an operational webhook endpoint.
    pub async fn update(
        &self,
        endpoint_id: String,
        operational_webhook_endpoint_update: OperationalWebhookEndpointUpdate,
    ) -> Result<OperationalWebhookEndpointOut> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(operational_webhook_endpoint_update)
        .execute(self.cfg)
        .await
    }

    /// Delete an operational webhook endpoint.
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

    /// Get the additional headers to be sent with the operational webhook.
    pub async fn get_headers(
        &self,
        endpoint_id: String,
    ) -> Result<OperationalWebhookEndpointHeadersOut> {
        crate::request::Request::new(
            http1::Method::GET,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .execute(self.cfg)
        .await
    }

    /// Set the additional headers to be sent with the operational webhook.
    pub async fn update_headers(
        &self,
        endpoint_id: String,
        operational_webhook_endpoint_headers_in: OperationalWebhookEndpointHeadersIn,
    ) -> Result<()> {
        crate::request::Request::new(
            http1::Method::PUT,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .with_body_param(operational_webhook_endpoint_headers_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }

    /// Get an operational webhook endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
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

    /// Rotates an operational webhook endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
    pub async fn rotate_secret(
        &self,
        endpoint_id: String,
        operational_webhook_endpoint_secret_in: OperationalWebhookEndpointSecretIn,
        options: Option<OperationalWebhookEndpointRotateSecretOptions>,
    ) -> Result<()> {
        let OperationalWebhookEndpointRotateSecretOptions { idempotency_key } =
            options.unwrap_or_default();

        crate::request::Request::new(
            http1::Method::POST,
            "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
        )
        .with_path_param("endpoint_id", endpoint_id)
        .with_optional_header_param("idempotency-key", idempotency_key)
        .with_body_param(operational_webhook_endpoint_secret_in)
        .returns_nothing()
        .execute(self.cfg)
        .await
    }
}
