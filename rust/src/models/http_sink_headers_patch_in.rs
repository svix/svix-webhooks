// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct HttpSinkHeadersPatchIn {
    pub headers: std::collections::HashMap<String, String>,
}

impl HttpSinkHeadersPatchIn {
    pub fn new(headers: std::collections::HashMap<String, String>) -> Self {
        Self { headers }
    }
}
