// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventTypeFromOpenApi {
    /// The event type's name
    pub name: String,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,

    pub deprecated: bool,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,

    #[serde(rename = "featureFlags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flags: Option<Vec<String>>,
}

impl EventTypeFromOpenApi {
    pub fn new(name: String, description: String, deprecated: bool) -> Self {
        Self {
            name,
            description,
            schemas: None,
            deprecated,
            group_name: None,
            feature_flags: None,
        }
    }
}
