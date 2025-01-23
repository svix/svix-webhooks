// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IntegrationOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// The integ's ID
    pub id: String,

    pub name: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl IntegrationOut {
    pub fn new(created_at: String, id: String, name: String, updated_at: String) -> Self {
        Self {
            created_at,
            id,
            name,
            updated_at,
        }
    }
}
