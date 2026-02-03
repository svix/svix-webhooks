// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApplicationOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// The Application's ID.
    pub id: String,

    pub metadata: std::collections::HashMap<String, String>,

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

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl ApplicationOut {
    pub fn new(
        created_at: String,
        id: String,
        metadata: std::collections::HashMap<String, String>,
        name: String,
        updated_at: String,
    ) -> Self {
        #[allow(deprecated)]
        Self {
            created_at,
            id,
            metadata,
            name,
            rate_limit: None,
            throttle_rate: None,
            uid: None,
            updated_at,
        }
    }
}
