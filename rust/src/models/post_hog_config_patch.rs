// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PostHogConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(rename = "projectToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_token: Option<String>,
}

impl PostHogConfigPatch {
    pub fn new() -> Self {
        Self {
            domain: None,
            project_token: None,
        }
    }
}

impl Default for PostHogConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
