// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_created_event_data::EndpointCreatedEventData;

/// Sent when an endpoint is created.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointCreatedEvent {
    pub data: EndpointCreatedEventData,

    pub r#type: String,
}

impl EndpointCreatedEvent {
    pub fn new(data: EndpointCreatedEventData, r#type: String) -> Self {
        Self { data, r#type }
    }
}
