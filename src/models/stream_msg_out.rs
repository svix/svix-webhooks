// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamMsgOut {
    pub offset: u64,

    pub topic: String,

    pub value: Vec<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    pub timestamp: jiff::Timestamp,
}

impl StreamMsgOut {
    pub fn new(offset: u64, topic: String, value: Vec<u8>, timestamp: jiff::Timestamp) -> Self {
        Self {
            offset,
            topic,
            value,
            headers: None,
            timestamp,
        }
    }
}
