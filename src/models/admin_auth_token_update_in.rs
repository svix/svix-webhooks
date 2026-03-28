// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenUpdateIn {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_ms: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl AdminAuthTokenUpdateIn {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: None,
            expiry_ms: None,
            enabled: None,
        }
    }

    pub fn with_name(mut self, value: impl Into<Option<String>>) -> Self {
        self.name = value.into();
        self
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
