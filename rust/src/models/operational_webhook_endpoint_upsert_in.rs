// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OperationalWebhookEndpointUpsertIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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

    #[serde(rename = "filterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl OperationalWebhookEndpointUpsertIn {
    pub fn new(url: String) -> Self {
        Self {
            description: None,
            throttle_rate: None,
            uid: None,
            url,
            disabled: None,
            filter_types: None,
            metadata: None,
        }
    }
}
