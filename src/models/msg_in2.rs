// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgIn2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// Optional partition key. Messages with the same key are routed to the same partition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    pub value: Vec<u8>,
}

impl MsgIn2 {
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            headers: None,
            key: None,
            value,
        }
    }
}
