// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointMtlsConfigIn {
    /// A PEM encoded private key and X.509 certificate to identify the webhook
    /// sender.
    pub identity: String,
}

impl EndpointMtlsConfigIn {
    pub fn new(identity: String) -> Self {
        Self { identity }
    }
}
