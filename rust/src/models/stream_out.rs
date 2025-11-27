// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamOut {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    /// The stream's ID.
    pub id: String,

    pub metadata: std::collections::HashMap<String, String>,

    /// The stream's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The stream's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl StreamOut {
    pub fn new(
        created_at: String,
        id: String,
        metadata: std::collections::HashMap<String, String>,
        updated_at: String,
    ) -> Self {
        Self {
            created_at,
            id,
            metadata,
            name: None,
            uid: None,
            updated_at,
        }
    }
}
