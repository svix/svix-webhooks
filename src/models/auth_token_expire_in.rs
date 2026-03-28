// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenExpireIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub id: String,

    /// Milliseconds from now until the token expires. `None` means expire immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_ms: Option<u64>,
}

impl AuthTokenExpireIn {
    pub fn new(id: String) -> Self {
        Self {
            namespace: None,
            id,
            expiry_ms: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_expiry_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.expiry_ms = value.into();
        self
    }
}
