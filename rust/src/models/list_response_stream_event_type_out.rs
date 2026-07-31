// this file is @generated
use serde::{Deserialize, Serialize};

use super::stream_event_type_out::StreamEventTypeOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseStreamEventTypeOut {
    pub data: Vec<StreamEventTypeOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseStreamEventTypeOut {
    pub fn new(data: Vec<StreamEventTypeOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
