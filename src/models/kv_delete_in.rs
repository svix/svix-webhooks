// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct KvDeleteIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// If set, the delete only succeeds when the stored version matches this value.
    /// Use the `version` field from a prior `get` response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

impl KvDeleteIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            version: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_version(mut self, value: impl Into<Option<u64>>) -> Self {
        self.version = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KvDeleteIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    /// If set, the delete only succeeds when the stored version matches this value.
    /// Use the `version` field from a prior `get` response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}
