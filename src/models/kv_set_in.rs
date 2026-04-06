// this file is @generated
use serde::{Deserialize, Serialize};

use super::operation_behavior::OperationBehavior;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct KvSetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Time to live in milliseconds
    #[serde(
        rename = "ttl_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub ttl: Option<std::time::Duration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<OperationBehavior>,

    /// If set, the write only succeeds when the stored version matches this value.
    /// Use the `version` field from a prior `get` response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

impl KvSetIn {
    pub fn new() -> Self {
        Self {
            namespace: None,
            ttl: None,
            behavior: None,
            version: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_ttl(mut self, value: impl Into<Option<std::time::Duration>>) -> Self {
        self.ttl = value.into();
        self
    }

    pub fn with_behavior(mut self, value: impl Into<Option<OperationBehavior>>) -> Self {
        self.behavior = value.into();
        self
    }

    pub fn with_version(mut self, value: impl Into<Option<u64>>) -> Self {
        self.version = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KvSetIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,

    /// Time to live in milliseconds
    #[serde(
        rename = "ttl_ms",
        skip_serializing_if = "Option::is_none",
        with = "crate::duration_ms_serde::optional"
    )]
    pub ttl: Option<std::time::Duration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<OperationBehavior>,

    /// If set, the write only succeeds when the stored version matches this value.
    /// Use the `version` field from a prior `get` response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}
