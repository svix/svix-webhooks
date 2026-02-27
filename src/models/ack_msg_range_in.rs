// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AckMsgRangeIn {
    pub consumer_group: String,

    pub max_msg_id: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_msg_id: Option<u64>,

    pub name: String,
}

impl AckMsgRangeIn {
    pub fn new(consumer_group: String, max_msg_id: u64, name: String) -> Self {
        Self {
            consumer_group,
            max_msg_id,
            min_msg_id: None,
            name,
        }
    }
}
