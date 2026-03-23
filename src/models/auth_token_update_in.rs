// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenUpdateIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_millis: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl AuthTokenUpdateIn {
    pub fn new(id: String) -> Self {
        Self {
            namespace: None,
            id,
            name: None,
            expiry_millis: None,
            metadata: None,
            scopes: None,
            enabled: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_name(mut self, value: impl Into<Option<String>>) -> Self {
        self.name = value.into();
        self
    }

    pub fn with_expiry_millis(mut self, value: impl Into<Option<u64>>) -> Self {
        self.expiry_millis = value.into();
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
