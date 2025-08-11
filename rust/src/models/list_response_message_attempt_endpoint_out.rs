use serde::{Deserialize, Serialize};

use super::message_attempt_endpoint_out::MessageAttemptEndpointOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseMessageAttemptEndpointOut {
    pub data: Vec<MessageAttemptEndpointOut>,

    pub done: bool,

    pub iterator: String,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseMessageAttemptEndpointOut {
    pub fn new(data: Vec<MessageAttemptEndpointOut>, done: bool, iterator: String) -> Self {
        Self {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
