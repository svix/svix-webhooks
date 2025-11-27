// this file is @generated
use std::fmt;

use serde::{
    Deserialize,
    Serialize,
};

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
    #[serde(rename = "application.purge_content")]
    ApplicationPurgeContent,
}

impl fmt::Display for BackgroundTaskType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let value = match self {
            Self::EndpointReplay => "endpoint.replay",
            Self::EndpointRecover => "endpoint.recover",
            Self::ApplicationStats => "application.stats",
            Self::MessageBroadcast => "message.broadcast",
            Self::SdkGenerate => "sdk.generate",
            Self::EventTypeAggregate => "event-type.aggregate",
            Self::ApplicationPurgeContent => "application.purge_content",
        };
        f.write_str(value)
    }
}
