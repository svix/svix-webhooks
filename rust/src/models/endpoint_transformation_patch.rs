// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointTransformationPatch {
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub code: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub variables: JsOption<std::collections::BTreeMap<String, String>>,
}

impl EndpointTransformationPatch {
    pub fn new() -> Self {
        Self {
            code: JsOption::Undefined,
            enabled: None,
            variables: JsOption::Undefined,
        }
    }
}

impl Default for EndpointTransformationPatch {
    fn default() -> Self {
        Self::new()
    }
}
