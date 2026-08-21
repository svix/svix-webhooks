// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqConfigPatch {
    #[serde(rename = "routingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl RabbitMqConfigPatch {
    pub fn new() -> Self {
        Self {
            routing_key: None,
            uri: None,
        }
    }
}

impl Default for RabbitMqConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
