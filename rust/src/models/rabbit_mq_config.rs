// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a RabbitMq sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqConfig {
    pub uri: String,

    #[serde(rename = "routingKey")]
    pub routing_key: String,
}

impl RabbitMqConfig {
    pub fn new(uri: String, routing_key: String) -> Self {
        Self { uri, routing_key }
    }
}
