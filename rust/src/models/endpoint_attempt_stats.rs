// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointAttemptStats {
    pub success: u64,

    pub fail: u64,

    pub canceled: u64,
}

impl EndpointAttemptStats {
    pub fn new(success: u64, fail: u64, canceled: u64) -> Self {
        Self {
            success,
            fail,
            canceled,
        }
    }
}
