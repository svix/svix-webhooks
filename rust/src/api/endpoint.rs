use super::PostOptions;
use crate::{apis::endpoint_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EndpointListOptions {
    pub iterator: Option<String>,
    pub limit: Option<i32>,
    pub order: Option<Ordering>,
}

pub struct Endpoint<'a> {
    cfg: &'a Configuration,
}

#[derive(Default)]
pub struct EndpointStatsOptions {
    pub since: Option<String>,
    pub until: Option<String>,
}

impl<'a> Endpoint<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    pub async fn list(
        &self,
        app_id: String,
        options: Option<EndpointListOptions>,
    ) -> Result<ListResponseEndpointOut> {
        let EndpointListOptions {
            iterator,
            limit,
            order,
        } = options.unwrap_or_default();
        endpoint_api::v1_period_endpoint_period_list(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodListParams {
                app_id,
                order,
                iterator,
                limit,
            },
        )
        .await
    }

    pub async fn create(
        &self,
        app_id: String,
        endpoint_in: EndpointIn,
        options: Option<PostOptions>,
    ) -> Result<EndpointOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        endpoint_api::v1_period_endpoint_period_create(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodCreateParams {
                app_id,
                endpoint_in,
                idempotency_key,
            },
        )
        .await
    }

    pub async fn get(&self, app_id: String, endpoint_id: String) -> Result<EndpointOut> {
        endpoint_api::v1_period_endpoint_period_get(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodGetParams {
                app_id,
                endpoint_id,
            },
        )
        .await
    }

    pub async fn update(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_update: EndpointUpdate,
    ) -> Result<EndpointOut> {
        endpoint_api::v1_period_endpoint_period_update(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodUpdateParams {
                app_id,
                endpoint_id,
                endpoint_update,
            },
        )
        .await
    }

    pub async fn patch(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_patch: EndpointPatch,
    ) -> Result<EndpointOut> {
        endpoint_api::v1_period_endpoint_period_patch(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodPatchParams {
                app_id,
                endpoint_id,
                endpoint_patch,
            },
        )
        .await
    }

    pub async fn delete(&self, app_id: String, endpoint_id: String) -> Result<()> {
        endpoint_api::v1_period_endpoint_period_delete(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodDeleteParams {
                app_id,
                endpoint_id,
            },
        )
        .await
    }

    pub async fn get_secret(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointSecretOut> {
        endpoint_api::v1_period_endpoint_period_get_secret(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodGetSecretParams {
                app_id,
                endpoint_id,
            },
        )
        .await
    }

    pub async fn rotate_secret(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
    ) -> Result<()> {
        endpoint_api::v1_period_endpoint_period_rotate_secret(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodRotateSecretParams {
                app_id,
                endpoint_id,
                endpoint_secret_rotate_in,
                idempotency_key: None,
            },
        )
        .await
    }

    pub async fn recover(
        &self,
        app_id: String,
        endpoint_id: String,
        recover_in: RecoverIn,
    ) -> Result<()> {
        endpoint_api::v1_period_endpoint_period_recover(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodRecoverParams {
                app_id,
                endpoint_id,
                recover_in,
                idempotency_key: None,
            },
        )
        .await?;
        Ok(())
    }

    pub async fn get_headers(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointHeadersOut> {
        endpoint_api::v1_period_endpoint_period_get_headers(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodGetHeadersParams {
                app_id,
                endpoint_id,
            },
        )
        .await
    }

    pub async fn update_headers(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_headers_in: EndpointHeadersIn,
    ) -> Result<()> {
        endpoint_api::v1_period_endpoint_period_update_headers(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodUpdateHeadersParams {
                app_id,
                endpoint_id,
                endpoint_headers_in,
            },
        )
        .await
    }

    pub async fn patch_headers(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_headers_patch_in: EndpointHeadersPatchIn,
    ) -> Result<()> {
        endpoint_api::v1_period_endpoint_period_patch_headers(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodPatchHeadersParams {
                app_id,
                endpoint_id,
                endpoint_headers_patch_in,
            },
        )
        .await
    }

    pub async fn get_stats(
        &self,
        app_id: String,
        endpoint_id: String,
        options: Option<EndpointStatsOptions>,
    ) -> Result<EndpointStats> {
        let EndpointStatsOptions { since, until } = options.unwrap_or_default();
        endpoint_api::v1_period_endpoint_period_get_stats(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodGetStatsParams {
                app_id,
                endpoint_id,
                since,
                until,
            },
        )
        .await
    }

    pub async fn replay_missing(
        &self,
        app_id: String,
        endpoint_id: String,
        replay_in: ReplayIn,
        options: Option<PostOptions>,
    ) -> Result<()> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        endpoint_api::v1_period_endpoint_period_replay(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodReplayParams {
                app_id,
                endpoint_id,
                replay_in,
                idempotency_key,
            },
        )
        .await?;
        Ok(())
    }

    pub async fn transformation_get(
        &self,
        app_id: String,
        endpoint_id: String,
    ) -> Result<EndpointTransformationOut> {
        endpoint_api::v1_period_endpoint_period_transformation_get(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodTransformationGetParams {
                app_id,
                endpoint_id,
            },
        )
        .await
    }

    pub async fn transformation_partial_update(
        &self,
        app_id: String,
        endpoint_id: String,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> Result<()> {
        endpoint_api::v1_period_endpoint_period_transformation_partial_update(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodTransformationPartialUpdateParams {
                app_id,
                endpoint_id,
                endpoint_transformation_in,
            },
        )
        .await?;
        Ok(())
    }

    pub async fn send_example(
        &self,
        app_id: String,
        endpoint_id: String,
        event_example_in: EventExampleIn,
        options: Option<PostOptions>,
    ) -> Result<MessageOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();
        endpoint_api::v1_period_endpoint_period_send_example(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodSendExampleParams {
                app_id,
                endpoint_id,
                event_example_in,
                idempotency_key,
            },
        )
        .await
    }
}
