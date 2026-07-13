// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointOut {
    /// The Endpoint's ID.
    pub id: String,

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

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub metadata: std::collections::HashMap<String, String>,
}

impl IngestEndpointOut {
    pub fn new(
        id: String,
        description: String,
        url: String,
        created_at: String,
        updated_at: String,
        metadata: std::collections::HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            description,
            throttle_rate: None,
            uid: None,
            url,
            disabled: None,
            created_at,
            updated_at,
            metadata,
        }
    }
}
