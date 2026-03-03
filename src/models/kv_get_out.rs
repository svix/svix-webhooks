// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetOut {
    pub key: String,

    /// Time of expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<jiff::Timestamp>,

    pub value: Vec<u8>,
}

impl KvGetOut {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            key,
            expiry: None,
            value,
        }
    }
}
