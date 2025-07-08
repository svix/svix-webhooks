// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventTypeOut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    pub deprecated: bool,

    pub description: String,

    #[serde(rename = "featureFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The event type's name
    pub name: String,

    /// The schema for the event type for a specific version as a JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

impl EventTypeOut {
    pub fn new(
        created_at: String,
        deprecated: bool,
        description: String,
        name: String,
        updated_at: String,
    ) -> Self {
        Self {
            archived: None,
            created_at,
            deprecated,
            description,
            feature_flag: None,
            feature_flags: None,
            group_name: None,
            name,
            schemas: None,
            updated_at,
        }
    }
}
