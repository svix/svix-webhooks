// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_out::EndpointOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ListResponseEndpointOut {
    pub data: Vec<EndpointOut>,

    pub done: bool,

    pub iterator: String,

    #[serde(rename = "prevIterator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,
}

impl ListResponseEndpointOut {
    pub fn new(data: Vec<EndpointOut>, done: bool, iterator: String) -> Self {
        Self {
            data,
            done,
            iterator,
            prev_iterator: None,
        }
    }
}
