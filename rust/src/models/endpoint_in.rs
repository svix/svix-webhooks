// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointIn {
    /// List of message channels this endpoint listens to (omit for all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    #[serde(rename = "filterTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    /// Deprecated, use `throttleRate` instead.
    #[deprecated]
    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    /// The endpoint's verification secret.
    ///
    /// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    /// It is recommended to not set this and let the server generate the
    /// secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

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

    #[deprecated]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u16>,
}

impl EndpointIn {
    pub fn new(url: String) -> Self {
        #[allow(deprecated)]
        Self {
            channels: None,
            description: None,
            disabled: None,
            filter_types: None,
            headers: None,
            metadata: None,
            rate_limit: None,
            secret: None,
            throttle_rate: None,
            uid: None,
            url,
            version: None,
        }
    }
}
