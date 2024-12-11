use super::{Ordering, PostOptions};
use crate::{
    apis::webhook_endpoint_api as operational_webhook_endpoint_api, error::Result, models::*,
    Configuration,
};

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
        operational_webhook_endpoint_api::list_operational_webhook_endpoints(
            self.cfg,
            operational_webhook_endpoint_api::ListOperationalWebhookEndpointsParams {
                order,
                iterator,
                limit,
            },
        )
        .await
    }

    pub async fn create(
        &self,
        endpoint_in: OperationalWebhookEndpointIn,
        options: Option<PostOptions>,
    ) -> Result<OperationalWebhookEndpointOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        operational_webhook_endpoint_api::create_operational_webhook_endpoint(
            self.cfg,
            operational_webhook_endpoint_api::CreateOperationalWebhookEndpointParams {
                operational_webhook_endpoint_in: endpoint_in,
                idempotency_key,
            },
        )
        .await
    }

    pub async fn get(&self, endpoint_id: String) -> Result<OperationalWebhookEndpointOut> {
        operational_webhook_endpoint_api::get_operational_webhook_endpoint(
            self.cfg,
            operational_webhook_endpoint_api::GetOperationalWebhookEndpointParams { endpoint_id },
        )
        .await
    }

    pub async fn update(
        &self,
        endpoint_id: String,
        endpoint_update: OperationalWebhookEndpointUpdate,
    ) -> Result<OperationalWebhookEndpointOut> {
        operational_webhook_endpoint_api::update_operational_webhook_endpoint(
            self.cfg,
            operational_webhook_endpoint_api::UpdateOperationalWebhookEndpointParams {
                endpoint_id,
                operational_webhook_endpoint_update: endpoint_update,
            },
        )
        .await
    }

    pub async fn delete(&self, endpoint_id: String) -> Result<()> {
        operational_webhook_endpoint_api::delete_operational_webhook_endpoint(
            self.cfg,
            operational_webhook_endpoint_api::DeleteOperationalWebhookEndpointParams {
                endpoint_id,
            },
        )
        .await
    }

    pub async fn get_secret(
        &self,
        endpoint_id: String,
    ) -> Result<OperationalWebhookEndpointSecretOut> {
        operational_webhook_endpoint_api::get_operational_webhook_endpoint_secret(
            self.cfg,
            operational_webhook_endpoint_api::GetOperationalWebhookEndpointSecretParams {
                endpoint_id,
            },
        )
        .await
    }

    pub async fn rotate_secret(
        &self,
        endpoint_id: String,
        endpoint_secret_rotate_in: OperationalWebhookEndpointSecretIn,
    ) -> Result<()> {
        operational_webhook_endpoint_api::rotate_operational_webhook_endpoint_secret(
            self.cfg,
            operational_webhook_endpoint_api::RotateOperationalWebhookEndpointSecretParams {
                endpoint_id,
                operational_webhook_endpoint_secret_in: endpoint_secret_rotate_in,
                idempotency_key: None,
            },
        )
        .await
    }
}
