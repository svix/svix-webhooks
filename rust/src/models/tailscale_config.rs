// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TailscaleConfig {
    /// Shared secret for Tailscale Webhooks
    pub secret: String,

    /// Grace period (in seconds) for the timestamp.
    ///
    /// If not passed, timestamp age will not be checked.
    #[serde(rename = "timestampGraceSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_grace_seconds: Option<u32>,
}

impl TailscaleConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            timestamp_grace_seconds: None,
        }
    }
}
