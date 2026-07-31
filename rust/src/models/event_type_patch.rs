// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EventTypePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub schemas: JsOption<serde_json::Value>,

    #[serde(rename = "featureFlags")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flags: JsOption<std::collections::BTreeSet<String>>,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub group_name: JsOption<String>,
}

impl EventTypePatch {
    pub fn new() -> Self {
        Self {
            description: None,
            archived: None,
            deprecated: None,
            schemas: JsOption::Undefined,
            feature_flags: JsOption::Undefined,
            group_name: JsOption::Undefined,
        }
    }
}

impl Default for EventTypePatch {
    fn default() -> Self {
        Self::new()
    }
}
