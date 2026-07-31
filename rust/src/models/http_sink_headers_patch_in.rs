// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HttpSinkHeadersPatchIn {
    pub headers: std::collections::BTreeMap<String, String>,
}

impl HttpSinkHeadersPatchIn {
    pub fn new(headers: std::collections::BTreeMap<String, String>) -> Self {
        Self { headers }
    }
}
