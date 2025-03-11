// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_updated_event_data::EndpointUpdatedEventData;

/// Sent when an endpoint is updated.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointUpdatedEvent {
    pub data: EndpointUpdatedEventData,

    pub r#type: String,
}

impl EndpointUpdatedEvent {
    pub fn new(data: EndpointUpdatedEventData, r#type: String) -> Self {
        Self { data, r#type }
    }
}
