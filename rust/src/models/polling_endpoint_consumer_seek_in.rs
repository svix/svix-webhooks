// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollingEndpointConsumerSeekIn {
    pub after: chrono::DateTime<chrono::Utc>,
}

impl PollingEndpointConsumerSeekIn {
    pub fn new(after: chrono::DateTime<chrono::Utc>) -> Self {
        Self { after }
    }
}
