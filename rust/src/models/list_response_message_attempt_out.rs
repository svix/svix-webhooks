// this file is @generated
use serde::{Deserialize, Serialize};

use super::message_attempt_out::MessageAttemptOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseMessageAttemptOut {
    pub data: Vec<MessageAttemptOut>,

    pub done: bool,

    pub iterator: String,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseMessageAttemptOut {
    pub fn new(data: Vec<MessageAttemptOut>, done: bool, iterator: String) -> Self {
        Self {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
