// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Retention {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
}

impl Retention {
    pub fn new() -> Self {
        Self {
            ms: None,
            bytes: None,
        }
    }

    pub fn with_ms(mut self, value: impl Into<Option<u64>>) -> Self {
        self.ms = value.into();
        self
    }

    pub fn with_bytes(mut self, value: impl Into<Option<u64>>) -> Self {
        self.bytes = value.into();
        self
    }
}
