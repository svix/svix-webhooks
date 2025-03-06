// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestSourceConsumerPortalAccessIn {
    /// How long the token will be valid for, in seconds.
    ///
    /// Valid values are between 1 hour and 7 days. The default is 7 days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i32>,

    /// Whether the app portal should be in read-only mode.
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

impl IngestSourceConsumerPortalAccessIn {
    pub fn new() -> Self {
        Self {
            expiry: None,
            read_only: None,
        }
    }
}
