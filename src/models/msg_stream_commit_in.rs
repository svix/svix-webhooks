// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgStreamCommitIn {
    pub offset: u64,
}

impl MsgStreamCommitIn {
    pub fn new(offset: u64) -> Self {
        Self { offset }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgStreamCommitIn_ {
    pub topic: String,

    pub consumer_group: String,

    pub offset: u64,
}
