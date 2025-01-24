// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub enum BackgroundTaskType {
    #[default]
    #[serde(rename = "endpoint.replay")]
    EndpointReplay,
    #[serde(rename = "endpoint.recover")]
    EndpointRecover,
    #[serde(rename = "application.stats")]
    ApplicationStats,
    #[serde(rename = "message.broadcast")]
    MessageBroadcast,
    #[serde(rename = "sdk.generate")]
    SdkGenerate,
    #[serde(rename = "event-type.aggregate")]
    EventTypeAggregate,
}

impl fmt::Display for BackgroundTaskType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EndpointReplay => f.write_str("endpoint.replay"),
            Self::EndpointRecover => f.write_str("endpoint.recover"),
            Self::ApplicationStats => f.write_str("application.stats"),
            Self::MessageBroadcast => f.write_str("message.broadcast"),
            Self::SdkGenerate => f.write_str("sdk.generate"),
            Self::EventTypeAggregate => f.write_str("event-type.aggregate"),
        }
    }
}
