// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_disabled_event_data::EndpointDisabledEventData;

/// Sent when an endpoint has been automatically disabled after continuous
/// failures, or manually via an API call.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointDisabledEvent {
    pub data: EndpointDisabledEventData,

    pub r#type: String,
}

impl EndpointDisabledEvent {
    pub fn new(data: EndpointDisabledEventData, r#type: String) -> Self {
        Self { data, r#type }
    }
}
