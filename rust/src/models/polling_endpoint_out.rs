// this file is @generated
use serde::{Deserialize, Serialize};

use super::polling_endpoint_message_out::PollingEndpointMessageOut;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollingEndpointOut {
    pub data: Vec<PollingEndpointMessageOut>,

    pub iterator: String,

    pub done: bool,
}

impl PollingEndpointOut {
    pub fn new(data: Vec<PollingEndpointMessageOut>, iterator: String, done: bool) -> Self {
        Self {
            data,
            iterator,
            done,
        }
    }
}
