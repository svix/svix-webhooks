// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct IdempotencyStartIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// How long to hold the lock on start before releasing it.
    #[serde(rename = "lock_period_ms", with = "crate::duration_ms_serde")]
    pub lock_period: std::time::Duration,
}

impl IdempotencyStartIn {
    pub fn new(lock_period: std::time::Duration) -> Self {
        Self {
            namespace: None,
            lock_period,
        }
    }

    pub fn with_namespace(mut self, value: impl Into<Option<String>>) -> Self {
        self.namespace = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct IdempotencyStartIn_ {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    pub key: String,

    /// How long to hold the lock on start before releasing it.
    #[serde(rename = "lock_period_ms", with = "crate::duration_ms_serde")]
    pub lock_period: std::time::Duration,
}
