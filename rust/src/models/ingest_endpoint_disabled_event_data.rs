// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::endpoint_disabled_trigger::EndpointDisabledTrigger;

/// Sent when an ingest endpoint has been automatically disabled after
/// continuous failures, or manually via an API call.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointDisabledEventData {
    /// The Endpoint's ID.
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,

    /// The Endpoint's UID.
    #[serde(rename = "endpointUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_uid: Option<String>,

    #[serde(rename = "failSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_since: Option<String>,

    /// The Source's ID.
    #[serde(rename = "sourceId")]
    pub source_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<EndpointDisabledTrigger>,
}

impl IngestEndpointDisabledEventData {
    pub fn new(
        endpoint_id: String,
        source_id: String,
    ) -> Self {
        Self {
            endpoint_id,
            endpoint_uid: None,
            fail_since: None,
            source_id,
            trigger: None,
        }
    }
}
