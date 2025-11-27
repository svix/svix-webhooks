// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamPortalAccessIn {
    /// How long the token will be valid for, in seconds.
    ///
    /// Valid values are between 1 hour and 7 days. The default is 7 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i32>,

    /// The set of feature flags the created token will have access to.
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    /// An optional session ID to attach to the token.
    ///
    /// When expiring tokens with "Expire All", you can include the session ID
    /// to only expire tokens that were created with that session ID.
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl StreamPortalAccessIn {
    pub fn new() -> Self {
        Self {
            expiry: None,
            feature_flags: None,
            session_id: None,
        }
    }
}
