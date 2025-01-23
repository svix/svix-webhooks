// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventTypeFromOpenApi {
    pub deprecated: bool,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,
}

impl EventTypeFromOpenApi {
    pub fn new(deprecated: bool, description: String, name: String) -> Self {
        Self {
            deprecated,
            description,
            feature_flag: None,
            group_name: None,
            name,
            schemas: None,
        }
    }
}
