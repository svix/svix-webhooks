// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ack {
    pub consumer_group: String,

    pub msg_id: u64,

    pub name: String,
}

impl Ack {
    pub fn new(consumer_group: String, msg_id: u64, name: String) -> Self {
        Self {
            consumer_group,
            msg_id,
            name,
        }
    }
}
