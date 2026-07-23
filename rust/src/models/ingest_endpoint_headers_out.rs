// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointHeadersOut {
    pub headers: std::collections::BTreeMap<String, String>,

    pub sensitive: std::collections::BTreeSet<String>,
}

impl IngestEndpointHeadersOut {
    pub fn new(
        headers: std::collections::BTreeMap<String, String>,
        sensitive: std::collections::BTreeSet<String>,
    ) -> Self {
        Self { headers, sensitive }
    }
}
