// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointHeadersIn {
    pub headers: std::collections::BTreeMap<String, String>,
}

impl EndpointHeadersIn {
    pub fn new(headers: std::collections::BTreeMap<String, String>) -> Self {
        Self { headers }
    }
}
