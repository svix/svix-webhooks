// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgTopicConfigureIn {
    pub partitions: u16,
}

impl MsgTopicConfigureIn {
    pub fn new(partitions: u16) -> Self {
        Self { partitions }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgTopicConfigureIn_ {
    pub topic: String,

    pub partitions: u16,
}
