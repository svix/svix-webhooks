// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamEventTypeIn {
    /// The event type's name
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl StreamEventTypeIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            feature_flags: None,
            deprecated: None,
            archived: None,
        }
    }
}
