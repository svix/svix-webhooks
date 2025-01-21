// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_message_out::EndpointMessageOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseEndpointMessageOut {
    pub data: Vec<EndpointMessageOut>,

    pub done: bool,

    pub iterator: String,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseEndpointMessageOut {
    pub fn new(data: Vec<EndpointMessageOut>, done: bool, iterator: String) -> Self {
        Self {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
