// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreateAutoConfigSubscriptionIn {
    /// The set of feature flags the created token will have access to.
    ///
    /// When omitted or empty, the token inherits the calling token's feature
    /// flags. When set, these flags are used instead. An application token
    /// may only grant a subset of its own flags.
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<std::collections::BTreeSet<String>>,
}

impl CreateAutoConfigSubscriptionIn {
    pub fn new() -> Self {
        Self {
            feature_flags: None,
        }
    }
}

impl Default for CreateAutoConfigSubscriptionIn {
    fn default() -> Self {
        Self::new()
    }
}
