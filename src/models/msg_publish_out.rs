// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_publish_out_topic::MsgPublishOutTopic;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgPublishOut {
    pub topics: Vec<MsgPublishOutTopic>,
}

impl MsgPublishOut {
    pub fn new(topics: Vec<MsgPublishOutTopic>) -> Self {
        Self { topics }
    }
}
