// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IntegrationOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// The set of feature flags the integration has access to.
    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    /// The Integration's ID.
    pub id: String,

    pub name: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl IntegrationOut {
    pub fn new(
        created_at: String,
        id: String,
        name: String,
        updated_at: String,
    ) -> Self {
        Self {
            created_at,
            feature_flags: None,
            id,
            name,
            updated_at,
        }
    }
}
