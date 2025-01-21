// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventTypeIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    pub description: String,

    #[serde(rename = "featureFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<String>,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    /// The event type's name
    pub name: String,

    /// The schema for the event type for a specific version as a JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,
}

impl EventTypeIn {
    pub fn new(description: String, name: String) -> Self {
        Self {
            archived: None,
            deprecated: None,
            description,
            feature_flag: None,
            group_name: None,
            name,
            schemas: None,
        }
    }
}
