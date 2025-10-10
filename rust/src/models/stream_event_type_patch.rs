// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamEventTypePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub description: JsOption<String>,

    #[serde(rename = "featureFlags")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flags: JsOption<Vec<String>>,

    /// The event type's name
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub name: JsOption<String>,
}

impl StreamEventTypePatch {
    pub fn new() -> Self {
        Self {
            archived: None,
            deprecated: None,
            description: JsOption::Undefined,
            feature_flags: JsOption::Undefined,
            name: JsOption::Undefined,
        }
    }
}
