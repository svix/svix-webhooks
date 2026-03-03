// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgTopicConfigureOut {
    pub partitions: u16,
}

impl MsgTopicConfigureOut {
    pub fn new(partitions: u16) -> Self {
        Self { partitions }
    }
}
