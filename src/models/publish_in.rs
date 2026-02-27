// this file is @generated
use serde::{Deserialize, Serialize};

use super::msg_in2::MsgIn2;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublishIn {
    pub msgs: Vec<MsgIn2>,

    pub name: String,

    pub topic: String,
}

impl PublishIn {
    pub fn new(msgs: Vec<MsgIn2>, name: String, topic: String) -> Self {
        Self { msgs, name, topic }
    }
}
