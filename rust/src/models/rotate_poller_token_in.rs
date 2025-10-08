// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RotatePollerTokenIn {
    /// How long the token will be valid for, in seconds. Can be up to
    /// 31,536,000 seconds (1 year).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i32>,

    /// Updates the previous token's expiration, in seconds.
    ///
    /// If set to 0, the old token will immediately be revoked. Must be between
    /// 0 and 86,400 seconds (1 day).
    ///
    /// Defaults to 300 seconds (5 minutes).
    #[serde(rename = "oldTokenExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_token_expiry: Option<i32>,
}

impl RotatePollerTokenIn {
    pub fn new() -> Self {
        Self {
            expiry: None,
            old_token_expiry: None,
        }
    }
}
