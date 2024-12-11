use super::PostOptions;
use crate::{apis::endpoint_api, error::Result, models::*, Configuration};

#[derive(Default)]
pub struct EndpointListOptions {
    /// Limit the number of returned items
    pub limit: Option<i32>,

    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,

    /// The sorting order of the returned items
    pub order: Option<Ordering>,
}

#[derive(Default)]
pub struct EndpointStatsOptions {
    /// Filter the range to data starting from this date
    ///
    /// RFC3339 date string.
    pub since: Option<String>,

    /// Filter the range to data ending by this date
    ///
    /// RFC3339 date string.
    pub until: Option<String>,
}

pub struct Endpoint<'a> {
    cfg: &'a Configuration,
}

impl<'a> Endpoint<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// List the application's endpoints.
    pub async fn list(
        &self,
        app_id: String,
        options: Option<EndpointListOptions>,
    ) -> Result<ListResponseEndpointOut> {
        let EndpointListOptions {
            limit,
            iterator,
            order,
        } = options.unwrap_or_default();

        endpoint_api::v1_period_endpoint_period_list(
            self.cfg,
            endpoint_api::V1PeriodEndpointPeriodListParams {
                app_id,
                limit,
                iterator,
                order,
            },
        )
        .await
    }

    /// Create a new endpoint for the application.
    ///
    /// When `secret` is `null` the secret is automatically generated
    /// (recommended)
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

    /// Get an endpoint.
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

    /// Update an endpoint.
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

    /// Delete an endpoint.
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

    /// Partially update an endpoint.
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

    /// Get the endpoint's signing secret.
    ///
    /// This is used to verify the authenticity of the webhook.
    /// For more information please refer to
    /// [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
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

    /// Rotates the endpoint's signing secret.
    ///
    /// The previous secret will remain valid for the next 24 hours.
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

    /// Resend all failed messages since a given time.
    ///
    /// Messages that were sent successfully, even if failed initially, are not
    /// resent.
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

    /// Get the additional headers to be sent with the webhook
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

    /// Set the additional headers to be sent with the webhook
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

    /// Partially set the additional headers to be sent with the webhook
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

    /// Get basic statistics for the endpoint.
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

    /// Replays messages to the endpoint.
    ///
    /// Only messages that were created after `since` will be sent. Messages
    /// that were previously sent to the endpoint are not resent.
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

    /// Get the transformation code associated with this endpoint
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

    /// Set or unset the transformation code associated with this endpoint
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
        .await
    }

    /// Send an example message for an event
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
