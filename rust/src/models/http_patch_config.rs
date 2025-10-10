// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct HttpPatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl HttpPatchConfig {
    pub fn new() -> Self {
        Self { url: None }
    }
}
