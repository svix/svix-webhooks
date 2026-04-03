// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenUpdateIn {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "expiry_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub expiry: Option<std::time::Duration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl AdminAuthTokenUpdateIn {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            expiry: None,
            enabled: None,
        }
    }

    pub fn with_name(mut self, value: impl Into<Option<String>>) -> Self {
        self.name = value.into();
        self
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
