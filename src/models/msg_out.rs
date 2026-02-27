// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgOut {
    pub headers: std::collections::HashMap<String, String>,

    pub id: u64,

    pub payload: Vec<u8>,

    pub timestamp: jiff::Timestamp,
}

impl MsgOut {
    pub fn new(
        headers: std::collections::HashMap<String, String>,
        id: u64,
        payload: Vec<u8>,
        timestamp: jiff::Timestamp,
    ) -> Self {
        Self {
            headers,
            id,
            payload,
            timestamp,
        }
    }
}
