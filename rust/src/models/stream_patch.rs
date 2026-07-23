// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamPatch {
    /// The Stream's description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An optional unique identifier for the stream.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uid: JsOption<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
}

impl StreamPatch {
    pub fn new() -> Self {
        Self {
            description: None,
            uid: JsOption::Undefined,
            metadata: None,
        }
    }
}

impl Default for StreamPatch {
    fn default() -> Self {
        Self::new()
    }
}
