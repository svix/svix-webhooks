// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamOut {
    /// The stream's ID.
    pub id: String,

    /// The stream's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// The stream's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub metadata: std::collections::BTreeMap<String, String>,
}
