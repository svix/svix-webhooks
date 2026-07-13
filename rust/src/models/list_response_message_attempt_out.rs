// this file is @generated
use serde::{Deserialize, Serialize};

use super::message_attempt_out::MessageAttemptOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseMessageAttemptOut {
    pub data: Vec<MessageAttemptOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseMessageAttemptOut {
    pub fn new(data: Vec<MessageAttemptOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
