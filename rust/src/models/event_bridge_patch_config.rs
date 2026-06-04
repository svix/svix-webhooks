// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventBridgePatchConfig {
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,

    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
}

impl EventBridgePatchConfig {
    pub fn new() -> Self {
        Self {
            access_key_id: None,
            detail_type: None,
            event_bus_name: None,
            region: None,
            secret_access_key: None,
        }
    }
}
