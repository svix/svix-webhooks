use super::PostOptions;
use crate::{apis::statistics_api, error::Result, models::*, Configuration};

pub struct Statistics<'a> {
    cfg: &'a Configuration,
}

pub struct AggregateAppStatsOptions {
    pub app_ids: Option<Vec<String>>,
    pub since: String,
    pub until: String,
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
        AggregateAppStatsOptions {
            app_ids,
            since,
            until,
        }: AggregateAppStatsOptions,
        options: Option<PostOptions>,
    ) -> Result<AppUsageStatsOut> {
        let options = options.unwrap_or_default();
        let params = statistics_api::V1PeriodStatisticsPeriodAggregateAppStatsParams {
            app_usage_stats_in: AppUsageStatsIn {
                app_ids,
                since,
                until,
            },
            idempotency_key: options.idempotency_key,
        };
        statistics_api::v1_period_statistics_period_aggregate_app_stats(self.cfg, params).await
    }

    /// Creates a background task to calculate the listed event types for all
    /// apps in the organization.
    ///
    /// Note that this endpoint is asynchronous. You will need to poll the `Get
    /// Background Task` endpoint to retrieve the results of the operation.
    pub async fn aggregate_event_types(&self) -> Result<AggregateEventTypesOut> {
        statistics_api::v1_period_statistics_period_aggregate_event_types(self.cfg).await
    }
}
