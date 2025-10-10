// this file is @generated
use serde::{Deserialize, Serialize};

use super::stream_sink_out::StreamSinkOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseStreamSinkOut {
    pub data: Vec<StreamSinkOut>,

    pub done: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseStreamSinkOut {
    pub fn new(data: Vec<StreamSinkOut>, done: bool) -> Self {
        Self {
            data,
            done,
            iterator: None,
            prev_iterator: None,
        }
    }
}
