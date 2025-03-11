// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_enabled_event_data::EndpointEnabledEventData;

/// Sent when an endpoint has been enabled.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointEnabledEvent {
    pub data: EndpointEnabledEventData,

    pub r#type: String,
}

impl EndpointEnabledEvent {
    pub fn new(data: EndpointEnabledEventData, r#type: String) -> Self {
        Self { data, r#type }
    }
}
