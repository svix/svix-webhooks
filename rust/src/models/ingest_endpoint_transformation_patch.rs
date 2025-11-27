// this file is @generated
use js_option::JsOption;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointTransformationPatch {
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub code: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl IngestEndpointTransformationPatch {
    pub fn new() -> Self {
        Self {
            code: JsOption::Undefined,
            enabled: None,
        }
    }
}
