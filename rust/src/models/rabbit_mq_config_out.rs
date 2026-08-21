// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RabbitMqConfigOut {
    #[serde(rename = "routingKey")]
    pub routing_key: String,
}
