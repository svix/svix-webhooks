// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApplicationIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    /// Application name for human consumption.
    pub name: String,

    /// Deprecated, use `throttleRate` instead.
    #[deprecated]
    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    /// Maximum messages per second to send to this application.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<u16>,

    /// Optional unique identifier for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl ApplicationIn {
    pub fn new(name: String) -> Self {
        #[allow(deprecated)]
        Self {
            metadata: None,
            name,
            rate_limit: None,
            throttle_rate: None,
            uid: None,
        }
    }
}
