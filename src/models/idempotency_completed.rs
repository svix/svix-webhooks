// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyCompleted {
    #[serde(with = "serde_bytes")]
    pub response: Vec<u8>,
}

impl IdempotencyCompleted {
    pub fn new(response: Vec<u8>) -> Self {
        Self { response }
    }
}
