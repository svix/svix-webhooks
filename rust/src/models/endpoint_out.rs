// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointOut {
    /// The Endpoint's ID.
    pub id: String,

    pub metadata: std::collections::BTreeMap<String, String>,

    pub url: String,

    pub description: String,

    /// Maximum messages per second to send to this endpoint.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<u16>,

    /// Optional unique identifier for the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<std::collections::BTreeSet<String>>,

    /// List of message channels this endpoint listens to (omit for all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<std::collections::BTreeSet<String>>,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl EndpointOut {
    pub fn new(
        id: String,
        metadata: std::collections::BTreeMap<String, String>,
        url: String,
        description: String,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            metadata,
            url,
            description,
            throttle_rate: None,
            uid: None,
            disabled: None,
            event_types: None,
            channels: None,
            created_at,
            updated_at,
        }
    }
}
