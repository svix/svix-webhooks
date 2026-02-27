// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheDeleteOut {
    pub deleted: bool,
}

impl CacheDeleteOut {
    pub fn new(deleted: bool) -> Self {
        Self { deleted }
    }
}
