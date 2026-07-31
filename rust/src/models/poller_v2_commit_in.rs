// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollerV2CommitIn {
    pub offset: u64,
}

impl PollerV2CommitIn {
    pub fn new(offset: u64) -> Self {
        Self { offset }
    }
}
