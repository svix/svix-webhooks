// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollingEndpointConsumerSeekOut {
    pub iterator: String,
}

impl PollingEndpointConsumerSeekOut {
    pub fn new(iterator: String) -> Self {
        Self { iterator }
    }
}
