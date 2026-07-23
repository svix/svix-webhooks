// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventTypeUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    pub description: String,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The schema for the event type for a specific version as a JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,
}

impl EventTypeUpdate {
    pub fn new(description: String) -> Self {
        Self {
            archived: None,
            deprecated: None,
            description,
            feature_flags: None,
            group_name: None,
            schemas: None,
        }
    }
}
