// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamEventTypeOut {
    /// The event type's name
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    pub deprecated: bool,

    pub archived: bool,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<std::collections::BTreeSet<String>>,
}

impl StreamEventTypeOut {
    pub fn new(
        name: String,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
        deprecated: bool,
        archived: bool,
    ) -> Self {
        Self {
            name,
            description: None,
            created_at,
            updated_at,
            deprecated,
            archived,
            feature_flags: None,
        }
    }
}
