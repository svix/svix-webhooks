// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenExpireIn {
    pub id: String,

    /// Milliseconds from now until the token expires. `None` means expire immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_ms: Option<u64>,
}

impl AdminAuthTokenExpireIn {
    pub fn new(id: String) -> Self {
        Self {
            id,
            expiry_ms: None,
        }
    }

    pub fn with_expiry_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.expiry_ms = value.into();
        self
    }
}
