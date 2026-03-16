// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetOut {
    /// Time of expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<jiff::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<u8>>,
}

impl KvGetOut {
    pub fn new() -> Self {
        Self {
            expiry: None,
            value: None,
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
