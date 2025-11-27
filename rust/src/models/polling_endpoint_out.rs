// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

use super::polling_endpoint_message_out::PollingEndpointMessageOut;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollingEndpointOut {
    pub data: Vec<PollingEndpointMessageOut>,

    pub done: bool,

    pub iterator: String,
}

impl PollingEndpointOut {
    pub fn new(
        data: Vec<PollingEndpointMessageOut>,
        done: bool,
        iterator: String,
    ) -> Self {
        Self {
            data,
            done,
            iterator,
        }
    }
}
