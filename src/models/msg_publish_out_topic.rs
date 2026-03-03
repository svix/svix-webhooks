// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgPublishOutTopic {
    pub topic: String,

    pub start_offset: u64,

    pub offset: u64,
}

impl MsgPublishOutTopic {
    pub fn new(topic: String, start_offset: u64, offset: u64) -> Self {
        Self {
            topic,
            start_offset,
            offset,
        }
    }
}
