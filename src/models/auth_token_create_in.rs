// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenCreateIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// Milliseconds from now until the token expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_ms: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    pub owner_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    /// Whether the token is enabled. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl AuthTokenCreateIn {
    pub fn new(name: String, owner_id: String) -> Self {
        Self {
            namespace: None,
            name,
            prefix: None,
            suffix: None,
            expiry_ms: None,
            metadata: None,
            owner_id,
            scopes: None,
            enabled: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_prefix(mut self, value: impl Into<Option<String>>) -> Self {
        self.prefix = value.into();
        self
    }

    pub fn with_suffix(mut self, value: impl Into<Option<String>>) -> Self {
        self.suffix = value.into();
        self
    }

    pub fn with_expiry_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.expiry_ms = value.into();
        self
    }

    pub fn with_metadata(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.metadata = value.into();
        self
    }

    pub fn with_scopes(mut self, value: impl Into<Option<Vec<String>>>) -> Self {
        self.scopes = value.into();
        self
    }

    pub fn with_enabled(mut self, value: impl Into<Option<bool>>) -> Self {
        self.enabled = value.into();
        self
    }
}
