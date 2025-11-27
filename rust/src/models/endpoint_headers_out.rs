// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

/// The value of the headers is returned in the `headers` field.
///
/// Sensitive headers that have been redacted are returned in the sensitive
/// field.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointHeadersOut {
    pub headers: std::collections::HashMap<String, String>,

    pub sensitive: Vec<String>,
}

impl EndpointHeadersOut {
    pub fn new(
        headers: std::collections::HashMap<String, String>,
        sensitive: Vec<String>,
    ) -> Self {
        Self {
            headers,
            sensitive,
        }
    }
}
