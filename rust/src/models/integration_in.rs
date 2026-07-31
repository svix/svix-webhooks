// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IntegrationIn {
    pub name: String,

    /// The set of feature flags the integration will have access to.
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<std::collections::BTreeSet<String>>,
}

impl IntegrationIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            feature_flags: None,
        }
    }
}
