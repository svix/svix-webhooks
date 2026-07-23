// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OperationalWebhookEndpointHeadersOut {
    pub headers: std::collections::BTreeMap<String, String>,

    pub sensitive: std::collections::BTreeSet<String>,
}

impl OperationalWebhookEndpointHeadersOut {
    pub fn new(
        headers: std::collections::BTreeMap<String, String>,
        sensitive: std::collections::BTreeSet<String>,
    ) -> Self {
        Self { headers, sensitive }
    }
}
