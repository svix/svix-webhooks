// this file is @generated
use serde::{Deserialize, Serialize};

use super::poller_v2_message_out::PollerV2MessageOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollerV2PollOut {
    pub data: Vec<PollerV2MessageOut>,

    pub done: bool,
}

impl PollerV2PollOut {
    pub fn new(data: Vec<PollerV2MessageOut>, done: bool) -> Self {
        Self { data, done }
    }
}
