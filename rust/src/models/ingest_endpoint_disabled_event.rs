// this file is @generated
use serde::{Deserialize, Serialize};

use super::ingest_endpoint_disabled_event_data::IngestEndpointDisabledEventData;

/// Sent when an ingest endpoint has been automatically disabled after
/// continuous failures, or manually via an API call.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointDisabledEvent {
    pub data: IngestEndpointDisabledEventData,

    pub r#type: String,
}

impl IngestEndpointDisabledEvent {
    pub fn new(data: IngestEndpointDisabledEventData, r#type: String) -> Self {
        Self { data, r#type }
    }
}
