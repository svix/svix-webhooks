// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct EndpointStats {
    pub canceled: i32,

    pub fail: i32,

    pub pending: i32,

    pub sending: i32,

    pub success: i32,
}

impl EndpointStats {
    pub fn new(canceled: i32, fail: i32, pending: i32, sending: i32, success: i32) -> Self {
        Self {
            canceled,
            fail,
            pending,
            sending,
            success,
        }
    }
}
