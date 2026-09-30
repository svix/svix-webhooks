// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a RabbitMq sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqConfigIn {
    /// URI to connect to
    ///
    /// Note that the VHost must be percent-escaped, so a default URI would look
    /// like `amqp://user:pass@host/%2F`
    pub uri: String,

    /// Routing key for message dispatch
    #[serde(rename = "routingKey")]
    pub routing_key: String,

    /// If true, then dispatches will fail if there is no attached queue; if
    /// false, they are silently dropped (this was previously the default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
}

impl RabbitMqConfigIn {
    pub fn new(uri: String, routing_key: String) -> Self {
        Self {
            uri,
            routing_key,
            mandatory: None,
        }
    }
}
