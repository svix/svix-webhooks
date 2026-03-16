// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueueMsgOut {
    pub msg_id: String,

    pub value: Vec<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    pub timestamp: jiff::Timestamp,
}

impl QueueMsgOut {
    pub fn new(msg_id: String, value: Vec<u8>, timestamp: jiff::Timestamp) -> Self {
        Self {
            msg_id,
            value,
            headers: None,
            timestamp,
        }
    }

    pub fn with_headers(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.headers = value.into();
        self
    }
}
