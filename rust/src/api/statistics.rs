use super::PostOptions;
use crate::{error::Result, models::*, Configuration};

pub struct Statistics<'a> {
    cfg: &'a Configuration,
}

impl<'a> Statistics<'a> {
    pub(super) fn new(cfg: &'a Configuration) -> Self {
        Self { cfg }
    }

    /// Creates a background task to calculate the message destinations for all
    /// applications in the environment.
    ///
    /// Note that this endpoint is asynchronous. You will need to poll the `Get
    /// Background Task` endpoint to retrieve the results of the operation.
    pub async fn aggregate_app_stats(
        &self,
        app_usage_stats_in: AppUsageStatsIn,
        options: Option<PostOptions>,
    ) -> Result<AppUsageStatsOut> {
        let PostOptions { idempotency_key } = options.unwrap_or_default();

        crate::request::Request::new(http1::Method::POST, "/api/v1/stats/usage/app")
            .with_body_param(app_usage_stats_in)
            .with_optional_header_param("idempotency-key", idempotency_key)
            .execute(self.cfg)
            .await
    }

    /// Creates a background task to calculate the listed event types for all
    /// apps in the organization.
    ///
    /// Note that this endpoint is asynchronous. You will need to poll the `Get
    /// Background Task` endpoint to retrieve the results of the operation.
    pub async fn aggregate_event_types(&self) -> Result<AggregateEventTypesOut> {
        crate::request::Request::new(http1::Method::PUT, "/api/v1/stats/usage/event-types")
            .execute(self.cfg)
            .await
    }
}
