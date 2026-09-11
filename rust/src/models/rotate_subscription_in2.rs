// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_secret_rotate_in::EndpointSecretRotateIn;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RotateSubscriptionIn2 {
    #[serde(rename = "signingSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<EndpointSecretRotateIn>,
}

impl RotateSubscriptionIn2 {
    pub fn new() -> Self {
        Self {
            signing_secret: None,
        }
    }
}

impl Default for RotateSubscriptionIn2 {
    fn default() -> Self {
        Self::new()
    }
}
