// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_disabled_trigger::EndpointDisabledTrigger;

/// Sent when an endpoint has been automatically disabled after continuous
/// failures, or manually via an API call.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointDisabledEventData {
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

    #[serde(rename = "failSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_since: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<EndpointDisabledTrigger>,
}

impl EndpointDisabledEventData {
    pub fn new(app_id: String, endpoint_id: String) -> Self {
        Self {
            app_id,
            app_uid: None,
            endpoint_id,
            endpoint_uid: None,
            fail_since: None,
            trigger: None,
        }
    }
}
