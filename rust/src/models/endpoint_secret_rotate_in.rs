// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointSecretRotateIn {
    /// The endpoint's verification secret.
    ///
    /// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    /// It is recommended to not set this and let the server generate the
    /// secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// How long the old secret will be valid for, in seconds.
    ///
    /// Valid values are between 0 (immediate expiry) and 7 days. The default is
    /// 24 hours.
    #[serde(rename = "gracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_period_seconds: Option<u32>,
}

impl EndpointSecretRotateIn {
    pub fn new() -> Self {
        Self {
            key: None,
            grace_period_seconds: None,
        }
    }
}

impl Default for EndpointSecretRotateIn {
    fn default() -> Self {
        Self::new()
    }
}
