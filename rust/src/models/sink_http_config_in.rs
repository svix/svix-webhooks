// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SinkHttpConfigIn {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl SinkHttpConfigIn {
    pub fn new(url: String) -> Self {
        Self {
            url,
            headers: None,
            key: None,
        }
    }
}
