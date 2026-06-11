// this file is @generated
use serde::{Deserialize, Serialize};

use super::{endpoint_in::EndpointIn, sink_in_common::SinkInCommon};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum AutoConfigSinkType {
    #[serde(rename = "poller")]
    Poller(SinkInCommon),
    #[serde(rename = "http")]
    Http(EndpointIn),
}

#[allow(clippy::derivable_impls)]
impl Default for AutoConfigSinkType {
    fn default() -> Self {
        Self::Poller(SinkInCommon::default())
    }
}
