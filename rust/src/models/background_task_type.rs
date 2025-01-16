use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum BackgroundTaskType {
    #[default]
    #[serde(rename = "endpoint.replay")]
    EndpointPeriodReplay,
    #[serde(rename = "endpoint.recover")]
    EndpointPeriodRecover,
    #[serde(rename = "application.stats")]
    ApplicationPeriodStats,
    #[serde(rename = "message.broadcast")]
    MessagePeriodBroadcast,
    #[serde(rename = "sdk.generate")]
    SdkPeriodGenerate,
    #[serde(rename = "event-type.aggregate")]
    EventTypePeriodAggregate,
}

impl std::fmt::Display for BackgroundTaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EndpointPeriodReplay => write!(f, "endpoint.replay"),
            Self::EndpointPeriodRecover => write!(f, "endpoint.recover"),
            Self::ApplicationPeriodStats => write!(f, "application.stats"),
            Self::MessagePeriodBroadcast => write!(f, "message.broadcast"),
            Self::SdkPeriodGenerate => write!(f, "sdk.generate"),
            Self::EventTypePeriodAggregate => write!(f, "event-type.aggregate"),
        }
    }
}
