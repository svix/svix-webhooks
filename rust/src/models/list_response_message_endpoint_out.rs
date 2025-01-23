// this file is @generated
use serde::{Deserialize, Serialize};

use super::message_endpoint_out::MessageEndpointOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseMessageEndpointOut {
    pub data: Vec<MessageEndpointOut>,

    pub done: bool,

    pub iterator: String,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseMessageEndpointOut {
    pub fn new(data: Vec<MessageEndpointOut>, done: bool, iterator: String) -> Self {
        Self {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
