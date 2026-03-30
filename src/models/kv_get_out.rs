// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetOut {
    /// Time of expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<jiff::Timestamp>,

    #[serde(
        with = "crate::serde_bytes_opt",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Vec<u8>>,

    /// Opaque version token for optimistic concurrency control.
    /// Pass as `version` in a subsequent `set` to perform a conditional write.
    pub version: u64,
}

impl KvGetOut {
    pub fn new(version: u64) -> Self {
        Self {
            expiry: None,
            value: None,
            version,
        }
    }

    pub fn with_expiry(mut self, value: impl Into<Option<jiff::Timestamp>>) -> Self {
        self.expiry = value.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<Option<Vec<u8>>>) -> Self {
        self.value = value.into();
        self
    }
}
