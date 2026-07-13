// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_message_out::EndpointMessageOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseEndpointMessageOut {
    pub data: Vec<EndpointMessageOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseEndpointMessageOut {
    pub fn new(data: Vec<EndpointMessageOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
