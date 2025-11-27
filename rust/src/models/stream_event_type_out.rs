// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamEventTypeOut {
    pub archived: bool,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    pub deprecated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    /// The event type's name
    pub name: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl StreamEventTypeOut {
    pub fn new(
        archived: bool,
        created_at: String,
        deprecated: bool,
        name: String,
        updated_at: String,
    ) -> Self {
        Self {
            archived,
            created_at,
            deprecated,
            description: None,
            feature_flags: None,
            name,
            updated_at,
        }
    }
}
