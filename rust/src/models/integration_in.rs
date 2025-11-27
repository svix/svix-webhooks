// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IntegrationIn {
    /// The set of feature flags the integration will have access to.
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    pub name: String,
}

impl IntegrationIn {
    pub fn new(name: String) -> Self {
        Self {
            feature_flags: None,
            name,
        }
    }
}
