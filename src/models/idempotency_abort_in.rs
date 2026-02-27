// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyAbortIn {
    pub key: String,
}

impl IdempotencyAbortIn {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
