// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OperationalWebhookEndpointOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// An example endpoint name.
    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "filterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<String>>,

    /// The Endpoint's ID.
    pub id: String,

    pub metadata: std::collections::HashMap<String, String>,

    /// Deprecated, use `throttleRate` instead.
    #[deprecated]
    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    /// Maximum messages per second to send to this endpoint.
    ///
    /// Outgoing messages will be throttled to this rate.
    #[serde(rename = "throttleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<u16>,

    /// Optional unique identifier for the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub url: String,
}

impl OperationalWebhookEndpointOut {
    pub fn new(
        created_at: String,
        description: String,
        id: String,
        metadata: std::collections::HashMap<String, String>,
        updated_at: String,
        url: String,
    ) -> Self {
        #[allow(deprecated)]
        Self {
            created_at,
            description,
            disabled: None,
            filter_types: None,
            id,
            metadata,
            rate_limit: None,
            throttle_rate: None,
            uid: None,
            updated_at,
            url,
        }
    }
}
