// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SegmentSinkConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    #[serde(rename = "writeKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_key: Option<String>,
}

impl SegmentSinkConfigPatch {
    pub fn new() -> Self {
        Self {
            host: None,
            write_key: None,
        }
    }
}

impl Default for SegmentSinkConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
