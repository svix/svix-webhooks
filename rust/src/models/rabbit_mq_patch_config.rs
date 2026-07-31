// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqPatchConfig {
    #[serde(rename = "routingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl RabbitMqPatchConfig {
    pub fn new() -> Self {
        Self {
            routing_key: None,
            uri: None,
        }
    }
}

impl Default for RabbitMqPatchConfig {
    fn default() -> Self {
        Self::new()
    }
}
