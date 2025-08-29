// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointHeadersPatchIn {
    /// A list of headers be be removed
    #[serde(rename = "deleteHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_headers: Option<Vec<String>>,

    pub headers: std::collections::HashMap<String, String>,
}

impl EndpointHeadersPatchIn {
    pub fn new(headers: std::collections::HashMap<String, String>) -> Self {
        Self {
            delete_headers: None,
            headers,
        }
    }
}
