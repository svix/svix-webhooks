// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct CacheDeleteIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

impl CacheDeleteIn {
    pub fn new() -> Self {
        Self { namespace: None }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct CacheDeleteIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,
}
