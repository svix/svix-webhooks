// this file is @generated
use serde::{Deserialize, Serialize};

use super::endpoint_in::EndpointIn;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SubscribeIn {
    pub endpoint: EndpointIn,
}

impl SubscribeIn {
    pub fn new(endpoint: EndpointIn) -> Self {
        Self { endpoint }
    }
}
