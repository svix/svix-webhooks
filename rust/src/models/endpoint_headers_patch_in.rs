// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointHeadersPatchIn {
    pub headers: std::collections::BTreeMap<String, String>,

    /// A list of headers be be removed
    #[serde(rename = "deleteHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_headers: Option<Vec<String>>,
}

impl EndpointHeadersPatchIn {
    pub fn new(headers: std::collections::BTreeMap<String, String>) -> Self {
        Self {
            headers,
            delete_headers: None,
        }
    }
}
