// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SinkHttpConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SinkHttpConfigPatch {
    pub fn new() -> Self {
        Self { url: None }
    }
}

impl Default for SinkHttpConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
