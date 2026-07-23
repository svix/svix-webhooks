// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OperationalWebhookEndpointHeadersIn {
    pub headers: std::collections::BTreeMap<String, String>,
}

impl OperationalWebhookEndpointHeadersIn {
    pub fn new(headers: std::collections::BTreeMap<String, String>) -> Self {
        Self { headers }
    }
}
