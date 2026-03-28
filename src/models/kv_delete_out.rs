// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvDeleteOut {
    /// Whether the operation succeeded or was a noop due to pre-conditions.
    pub success: bool,
}

impl KvDeleteOut {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
