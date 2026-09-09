// this file is @generated
use serde::{Deserialize, Serialize};

use super::destination_out::DestinationOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseDestinationOut {
    pub data: Vec<DestinationOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseDestinationOut {
    pub fn new(data: Vec<DestinationOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
