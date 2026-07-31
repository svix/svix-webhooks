// this file is @generated
use serde::{Deserialize, Serialize};

use super::stream_sink_out::StreamSinkOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseStreamSinkOut {
    pub data: Vec<StreamSinkOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseStreamSinkOut {
    pub fn new(data: Vec<StreamSinkOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
