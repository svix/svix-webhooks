// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_deleted_event_data::EndpointDeletedEventData;

/// Sent when an endpoint is deleted.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointDeletedEvent {
    pub data: EndpointDeletedEventData,

    pub r#type: String,
}

impl EndpointDeletedEvent {
    pub fn new(data: EndpointDeletedEventData, r#type: String) -> Self {
        Self { data, r#type }
    }
}
