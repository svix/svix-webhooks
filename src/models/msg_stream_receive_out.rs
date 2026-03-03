// this file is @generated
use serde::{Deserialize, Serialize};

use super::stream_msg_out::StreamMsgOut;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgStreamReceiveOut {
    pub msgs: Vec<StreamMsgOut>,
}

impl MsgStreamReceiveOut {
    pub fn new(msgs: Vec<StreamMsgOut>) -> Self {
        Self { msgs }
    }
}
