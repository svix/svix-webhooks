// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SinkOtelTracingConfigIn {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
}

impl SinkOtelTracingConfigIn {
    pub fn new(url: String) -> Self {
        Self { url, headers: None }
    }
}
