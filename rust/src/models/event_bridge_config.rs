// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventBridgeConfig {
    /// The name or ARN of the event bus to receive the event
    #[serde(rename = "eventBusName")]
    pub event_bus_name: String,

    /// Free-form string, with a maximum of 128 characters
    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,

    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    pub region: String,
}

impl EventBridgeConfig {
    pub fn new(
        event_bus_name: String,
        access_key_id: String,
        secret_access_key: String,
        region: String,
    ) -> Self {
        Self {
            event_bus_name,
            detail_type: None,
            access_key_id,
            secret_access_key,
            region,
        }
    }
}
