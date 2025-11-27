// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// An example endpoint name.
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// The Endpoint's ID.
    pub id: String,

    pub metadata: std::collections::HashMap<String, String>,

    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    /// Optional unique identifier for the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub url: String,
}

impl IngestEndpointOut {
    pub fn new(
        created_at: String,
        description: String,
        id: String,
        metadata: std::collections::HashMap<String, String>,
        updated_at: String,
        url: String,
    ) -> Self {
        Self {
            created_at,
            description,
            disabled: None,
            id,
            metadata,
            rate_limit: None,
            uid: None,
            updated_at,
            url,
        }
    }
}
