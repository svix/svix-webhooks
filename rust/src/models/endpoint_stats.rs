// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointStats {
    pub success: i64,

    pub pending: i64,

    pub sending: i64,

    pub fail: i64,

    pub canceled: i64,
}

impl EndpointStats {
    pub fn new(success: i64, pending: i64, sending: i64, fail: i64, canceled: i64) -> Self {
        Self {
            success,
            pending,
            sending,
            fail,
            canceled,
        }
    }
}
