// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HttpPatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl HttpPatchConfig {
    pub fn new() -> Self {
        Self { url: None }
    }
}

impl Default for HttpPatchConfig {
    fn default() -> Self {
        Self::new()
    }
}
