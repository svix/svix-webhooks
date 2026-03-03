// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in::MsgIn;

#[derive(Clone, Debug, Deserialize)]
pub struct MsgPublishIn {
    pub msgs: Vec<MsgIn>,
}

impl MsgPublishIn {
    pub fn new(msgs: Vec<MsgIn>) -> Self {
        Self { msgs }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgPublishIn_ {
    pub topic: String,

    pub msgs: Vec<MsgIn>,
}
