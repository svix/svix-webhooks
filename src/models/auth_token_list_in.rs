// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenListIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub owner_id: String,

    /// Limit the number of returned items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The iterator returned from a prior invocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,
}

impl AuthTokenListIn {
    pub fn new(owner_id: String) -> Self {
        Self {
            namespace: None,
            owner_id,
            limit: None,
            iterator: None,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }

    pub fn with_limit(mut self, value: impl Into<Option<u64>>) -> Self {
        self.limit = value.into();
        self
    }

    pub fn with_iterator(mut self, value: impl Into<Option<String>>) -> Self {
        self.iterator = value.into();
        self
    }
}
