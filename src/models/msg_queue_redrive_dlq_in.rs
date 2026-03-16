// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgQueueRedriveDlqIn {}

impl MsgQueueRedriveDlqIn {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgQueueRedriveDlqIn_ {
    pub topic: String,

    pub consumer_group: String,
}
