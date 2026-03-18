// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvSetOut {
    /// Whether the operation succeeded or was a noop due to pre-conditions.
    pub success: bool,

    pub version: u64,
}

impl KvSetOut {
    pub fn new(success: bool, version: u64) -> Self {
        Self { success, version }
    }
}
