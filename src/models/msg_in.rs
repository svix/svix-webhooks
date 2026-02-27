// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    pub payload: Vec<u8>,
}

impl MsgIn {
    pub fn new(payload: Vec<u8>) -> Self {
        Self {
            headers: None,
            payload,
        }
    }
}
