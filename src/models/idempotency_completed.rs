// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyCompleted {
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
}

impl IdempotencyCompleted {
    pub fn new(response: Vec<u8>) -> Self {
        Self {
            response,
            context: None,
        }
    }

    pub fn with_context(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.context = value.into();
        self
    }
}
