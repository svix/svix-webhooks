// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointHeadersIn {
    pub headers: std::collections::HashMap<String, String>,
}

impl EndpointHeadersIn {
    pub fn new(headers: std::collections::HashMap<String, String>) -> Self {
        Self {
            headers,
        }
    }
}
