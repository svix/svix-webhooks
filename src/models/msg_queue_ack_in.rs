// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgQueueAckIn {
    pub msg_ids: Vec<String>,
}

impl MsgQueueAckIn {
    pub fn new(msg_ids: Vec<String>) -> Self {
        Self { msg_ids }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgQueueAckIn_ {
    pub topic: String,

    pub consumer_group: String,

    pub msg_ids: Vec<String>,
}
