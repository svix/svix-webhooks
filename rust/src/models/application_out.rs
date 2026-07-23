// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ApplicationOut {
    /// Optional unique identifier for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// Application name for human consumption.
    pub name: String,

    /// Maximum messages per second to send to this application.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<u16>,

    /// The Application's ID.
    pub id: String,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub metadata: std::collections::BTreeMap<String, String>,
}

impl ApplicationOut {
    pub fn new(
        name: String,
        id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
        metadata: std::collections::BTreeMap<String, String>,
    ) -> Self {
        Self {
            uid: None,
            name,
            throttle_rate: None,
            id,
            created_at,
            updated_at,
            metadata,
        }
    }
}
