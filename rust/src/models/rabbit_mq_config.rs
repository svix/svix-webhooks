// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a RabbitMq sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqConfig {
    #[serde(rename = "routingKey")]
    pub routing_key: String,

    pub uri: String,
}

impl RabbitMqConfig {
    pub fn new(routing_key: String, uri: String) -> Self {
        Self { routing_key, uri }
    }
}
