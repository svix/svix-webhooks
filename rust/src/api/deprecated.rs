#[deprecated = "Use EndpointGetStatusOptions instead"]
pub type EndpointStatsOptions = super::EndpointGetStatsOptions;
#[deprecated = "Use MessageAttemptListByMsgOptions instead"]
pub type MessageAttemptListOptions = super::MessageAttemptListByMsgOptions;
#[deprecated = "Use MessageAttemptListAttemptedDestinationsOptions instead"]
pub type ListOptions = super::MessageAttemptListAttemptedDestinationsOptions;
#[deprecated = "Use AppUsageStatsIn instead"]
pub type AggregateAppStatsOptions = super::AppUsageStatsIn;

#[deprecated]
#[derive(Default)]
pub struct AuthenticationDashboardAccessOptions {
    pub idempotency_key: Option<String>,
}
