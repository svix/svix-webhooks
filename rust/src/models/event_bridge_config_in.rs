// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventBridgeConfigIn {
    /// The name or ARN of the event bus to receive the event
    #[serde(rename = "eventBusName")]
    pub event_bus_name: String,

    /// Free-form string, with a maximum of 128 characters
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,

    /// Access key ID.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    /// Secret access key.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    /// The region of the EventBridge bus.
    ///
    /// Currently a required field, but marked as optional because we may infer
    /// it from other fields in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl EventBridgeConfigIn {
    pub fn new(event_bus_name: String) -> Self {
        Self {
            event_bus_name,
            detail_type: None,
            access_key_id: None,
            secret_access_key: None,
            region: None,
        }
    }
}
