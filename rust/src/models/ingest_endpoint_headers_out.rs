// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointHeadersOut {
    pub headers: std::collections::BTreeMap<String, String>,

    pub sensitive: std::collections::BTreeSet<String>,
}
