// this file is @generated
use serde::{Deserialize, Serialize};

use super::queue_msg_out::QueueMsgOut;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgQueueReceiveOut {
    pub msgs: Vec<QueueMsgOut>,
}

impl MsgQueueReceiveOut {
    pub fn new(msgs: Vec<QueueMsgOut>) -> Self {
        Self { msgs }
    }
}
