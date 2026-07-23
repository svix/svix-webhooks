// this file is @generated
use serde::{Deserialize, Serialize};

/// The value of the headers is returned in the `headers` field.
///
/// Sensitive headers that have been redacted are returned in the sensitive
/// field.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointHeadersOut {
    pub headers: std::collections::BTreeMap<String, String>,

    pub sensitive: std::collections::BTreeSet<String>,
}
