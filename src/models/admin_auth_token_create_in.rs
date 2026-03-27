// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenCreateIn {
    pub name: String,

    pub role: String,

    /// Milliseconds from now until the token expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_ms: Option<u64>,

    /// Whether the token is enabled. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl AdminAuthTokenCreateIn {
    pub fn new(name: String, role: String) -> Self {
        Self {
            name,
            role,
            expiry_ms: None,
            enabled: None,
        }
    }

    pub fn with_expiry_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.expiry_ms = value.into();
        self
    }

    pub fn with_enabled(mut self, value: impl Into<Option<bool>>) -> Self {
        self.enabled = value.into();
        self
    }
}
