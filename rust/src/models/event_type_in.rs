// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventTypeIn {
    /// The event type's name
    pub name: String,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// The schema for the event type for a specific version as a JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,
}

impl EventTypeIn {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            archived: None,
            deprecated: None,
            schemas: None,
            group_name: None,
            feature_flags: None,
        }
    }
}
