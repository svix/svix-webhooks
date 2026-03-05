// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Retention {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
}

impl Retention {
    pub fn new() -> Self {
        Self {
            millis: None,
            bytes: None,
        }
    }

    pub fn with_millis(mut self, value: impl Into<Option<u64>>) -> Self {
        self.millis = value.into();
        self
    }

    pub fn with_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.bytes = value.into();
        self
    }
}
