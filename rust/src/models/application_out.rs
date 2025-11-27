// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ApplicationOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// The Application's ID.
    pub id: String,

    pub metadata: std::collections::HashMap<String, String>,

    pub name: String,

    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u16>,

    /// The Application's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl ApplicationOut {
    pub fn new(
        created_at: String,
        id: String,
        metadata: std::collections::HashMap<String, String>,
        name: String,
        updated_at: String,
    ) -> Self {
        Self {
            created_at,
            id,
            metadata,
            name,
            rate_limit: None,
            uid: None,
            updated_at,
        }
    }
}
