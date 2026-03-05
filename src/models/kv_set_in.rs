// this file is @generated
use serde::{Deserialize, Serialize};

use super::operation_behavior::OperationBehavior;

#[derive(Clone, Debug, Deserialize)]
pub struct KvSetIn {
    pub value: Vec<u8>,

    /// Time to live in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<OperationBehavior>,
}

impl KvSetIn {
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            ttl: None,
            behavior: None,
        }
    }

    pub fn with_ttl(mut self, value: impl Into<Option<u64>>) -> Self {
        self.ttl = value.into();
        self
    }

    pub fn with_behavior(mut self, value: impl Into<Option<OperationBehavior>>) -> Self {
        self.behavior = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KvSetIn_ {
    pub key: String,

    pub value: Vec<u8>,

    /// Time to live in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<OperationBehavior>,
}
