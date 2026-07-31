// this file is @generated
use serde::{Deserialize, Serialize};

use super::{auto_config_sink_type::AutoConfigSinkType, endpoint_in::EndpointIn};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubscribeIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<EndpointIn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sink: Option<AutoConfigSinkType>,
}

impl SubscribeIn {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            sink: None,
        }
    }
}

impl Default for SubscribeIn {
    fn default() -> Self {
        Self::new()
    }
}
