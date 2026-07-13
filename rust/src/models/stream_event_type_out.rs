// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamEventTypeOut {
    /// The event type's name
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    pub deprecated: bool,

    pub archived: bool,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,
}

impl StreamEventTypeOut {
    pub fn new(
        name: String,
        created_at: String,
        updated_at: String,
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
