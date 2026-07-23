// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EndpointStats {
    pub success: i32,

    pub pending: i32,

    pub sending: i32,

    pub fail: i32,

    pub canceled: i32,
}

impl EndpointStats {
    pub fn new(success: i32, pending: i32, sending: i32, fail: i32, canceled: i32) -> Self {
        Self {
            success,
            pending,
            sending,
            fail,
            canceled,
        }
    }
}
