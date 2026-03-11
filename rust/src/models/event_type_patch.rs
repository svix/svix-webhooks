// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EventTypePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Deprecated, use `featureFlags` instead.
    #[deprecated]
    #[serde(rename = "featureFlag")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flag: JsOption<String>,

    #[serde(rename = "featureFlags")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flags: JsOption<Vec<String>>,

    /// The event type group's name
    #[serde(rename = "groupName")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub group_name: JsOption<String>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub schemas: JsOption<serde_json::Value>,
}

impl EventTypePatch {
    pub fn new() -> Self {
        #[allow(deprecated)]
        Self {
            archived: None,
            deprecated: None,
            description: None,
            feature_flag: JsOption::Undefined,
            feature_flags: JsOption::Undefined,
            group_name: JsOption::Undefined,
            schemas: JsOption::Undefined,
        }
    }
}
