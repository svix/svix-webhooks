// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SinkHttpConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    pub url: String,
}

impl SinkHttpConfig {
    pub fn new(url: String) -> Self {
        Self {
            headers: None,
            key: None,
            url,
        }
    }
}
