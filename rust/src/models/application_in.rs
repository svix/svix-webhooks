// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ApplicationIn {
    /// Application name for human consumption.
    pub name: String,

    /// Maximum messages per second to send to this application.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<u16>,

    /// Optional unique identifier for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl ApplicationIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            throttle_rate: None,
            uid: None,
            metadata: None,
        }
    }
}
