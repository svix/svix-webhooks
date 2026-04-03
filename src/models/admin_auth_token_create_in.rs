// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenCreateIn {
    pub name: String,

    pub role: String,

    /// Milliseconds from now until the token expires.
    #[serde(
        rename = "expiry_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub expiry: Option<std::time::Duration>,

    /// Whether the token is enabled. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl AdminAuthTokenCreateIn {
    pub fn new(name: String, role: String) -> Self {
        Self {
            name,
            role,
            expiry: None,
            enabled: None,
        }
    }

    pub fn with_expiry(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.expiry = value.into();
        self
    }

    pub fn with_enabled(mut self, value: impl Into<Option<bool>>) -> Self {
        self.enabled = value.into();
        self
    }
}
