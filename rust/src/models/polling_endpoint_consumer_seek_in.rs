// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PollingEndpointConsumerSeekIn {
    pub after: String,
}

impl PollingEndpointConsumerSeekIn {
    pub fn new(after: String) -> Self {
        Self {
            after,
        }
    }
}
