// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::event_type_out::EventTypeOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseEventTypeOut {
    pub data: Vec<EventTypeOut>,

    pub done: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseEventTypeOut {
    pub fn new(
        data: Vec<EventTypeOut>,
        done: bool,
    ) -> Self {
        Self {
            data,
            done,
            iterator: None,
            prev_iterator: None,
        }
    }
}
