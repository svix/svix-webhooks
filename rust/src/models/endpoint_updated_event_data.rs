// this file is @generated
use serde::{Deserialize, Serialize};

/// Sent when an endpoint is created, updated, or deleted
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointUpdatedEventData {
    /// The Application's ID.
    #[serde(rename = "appId")]
    pub app_id: String,

    /// The Application's UID.
    #[serde(rename = "appUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_uid: Option<String>,

    /// The Endpoint's ID.
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,

    /// The Endpoint's UID.
    #[serde(rename = "endpointUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_uid: Option<String>,
}

impl EndpointUpdatedEventData {
    pub fn new(app_id: String, endpoint_id: String) -> Self {
        Self {
            app_id,
            app_uid: None,
            endpoint_id,
            endpoint_uid: None,
        }
    }
}
