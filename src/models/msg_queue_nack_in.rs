// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgQueueNackIn {
    pub msg_ids: Vec<String>,
}

impl MsgQueueNackIn {
    pub fn new(msg_ids: Vec<String>) -> Self {
        Self { msg_ids }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgQueueNackIn_ {
    pub topic: String,

    pub consumer_group: String,

    pub msg_ids: Vec<String>,
}
