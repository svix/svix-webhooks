// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IntegrationOut {
    pub name: String,

    /// The Integration's ID.
    pub id: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    /// The set of feature flags the integration has access to.
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,
}

impl IntegrationOut {
    pub fn new(name: String, id: String, created_at: String, updated_at: String) -> Self {
        Self {
            name,
            id,
            created_at,
            updated_at,
            feature_flags: None,
        }
    }
}
