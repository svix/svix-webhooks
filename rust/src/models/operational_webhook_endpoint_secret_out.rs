// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OperationalWebhookEndpointSecretOut {
    /// The endpoint's verification secret.
    ///
    /// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    /// It is recommended to not set this and let the server generate the
    /// secret.
    pub key: String,
}
