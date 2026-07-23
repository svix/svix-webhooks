// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventBridgeConfig {
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    /// Free-form string, with a maximum of 128 characters
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,

    /// The name or ARN of the event bus to receive the event
    #[serde(rename = "eventBusName")]
    pub event_bus_name: String,

    pub region: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
}

impl EventBridgeConfig {
    pub fn new(
        access_key_id: String,
        event_bus_name: String,
        region: String,
        secret_access_key: String,
    ) -> Self {
        Self {
            access_key_id,
            detail_type: None,
            event_bus_name,
            region,
            secret_access_key,
        }
    }
}
