// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueMsgOut {
    pub msg_id: String,

    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    pub timestamp: jiff::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<jiff::Timestamp>,
}

impl QueueMsgOut {
    pub fn new(msg_id: String, value: Vec<u8>, timestamp: jiff::Timestamp) -> Self {
        Self {
            msg_id,
            value,
            headers: None,
            timestamp,
            scheduled_at: None,
        }
    }

    pub fn with_headers(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.headers = value.into();
        self
    }

    pub fn with_scheduled_at(mut self, value: impl Into<Option<jiff::Timestamp>>) -> Self {
        self.scheduled_at = value.into();
        self
    }
}
