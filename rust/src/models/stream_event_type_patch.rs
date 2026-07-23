// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamEventTypePatch {
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub description: JsOption<String>,

    #[serde(rename = "featureFlags")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub feature_flags: JsOption<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

impl StreamEventTypePatch {
    pub fn new() -> Self {
        Self {
            description: JsOption::Undefined,
            feature_flags: JsOption::Undefined,
            deprecated: None,
            archived: None,
        }
    }
}

impl Default for StreamEventTypePatch {
    fn default() -> Self {
        Self::new()
    }
}
