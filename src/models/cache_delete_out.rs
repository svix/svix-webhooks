// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheDeleteOut {
    pub success: bool,
}

impl CacheDeleteOut {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
