// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct IngestEndpointSecretIn {
    /// The endpoint's verification secret.
    ///
    /// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    /// It is recommended to not set this and let the server generate the
    /// secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl IngestEndpointSecretIn {
    pub fn new() -> Self {
        Self { key: None }
    }
}
