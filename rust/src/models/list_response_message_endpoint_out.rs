// this file is @generated
use serde::{Deserialize, Serialize};

use super::message_endpoint_out::MessageEndpointOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListResponseMessageEndpointOut {
    pub data: Vec<MessageEndpointOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseMessageEndpointOut {
    pub fn new(data: Vec<MessageEndpointOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }
}
