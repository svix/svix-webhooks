// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PollerV2CommitIn {
    pub offset: i32,
}

impl PollerV2CommitIn {
    pub fn new(offset: i32) -> Self {
        Self { offset }
    }
}
