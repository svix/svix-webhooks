// this file is @generated
use serde::{Deserialize, Serialize};

use super::publish_out_msg::PublishOutMsg;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublishOut {
    pub msgs: Vec<PublishOutMsg>,
}

impl PublishOut {
    pub fn new(msgs: Vec<PublishOutMsg>) -> Self {
        Self { msgs }
    }
}
