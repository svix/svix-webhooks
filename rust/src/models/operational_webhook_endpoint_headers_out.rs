// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OperationalWebhookEndpointHeadersOut {
    pub headers: std::collections::HashMap<String, String>,

    pub sensitive: Vec<String>,
}

impl OperationalWebhookEndpointHeadersOut {
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
