// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetOut {
    /// Time of expiry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<jiff::Timestamp>,

    pub key: String,

    pub value: Vec<u8>,
}

impl KvGetOut {
    pub fn new(key: String, value: Vec<u8>) -> Self {
        Self {
            expiry: None,
            key,
            value,
        }
    }
}
