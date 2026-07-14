// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointOut {
    /// The Endpoint's ID.
    pub id: String,

    pub metadata: std::collections::HashMap<String, String>,

    /// An example endpoint name.
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

    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    /// List of message channels this endpoint listens to (omit for all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl EndpointOut {
    pub fn new(
        id: String,
        metadata: std::collections::HashMap<String, String>,
        description: String,
        url: String,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            metadata,
            description,
            throttle_rate: None,
            uid: None,
            url,
            disabled: None,
            event_types: None,
            channels: None,
            created_at,
            updated_at,
        }
    }
}
