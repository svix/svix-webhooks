// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventBridgeConfigPatch {
    #[serde(rename = "eventBusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,

    #[serde(rename = "detailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,

    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl EventBridgeConfigPatch {
    pub fn new() -> Self {
        Self {
            event_bus_name: None,
            detail_type: None,
            access_key_id: None,
            secret_access_key: None,
            region: None,
        }
    }
}

impl Default for EventBridgeConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
