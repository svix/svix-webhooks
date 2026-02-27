// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvDeleteOut {
    pub deleted: bool,
}

impl KvDeleteOut {
    pub fn new(deleted: bool) -> Self {
        Self { deleted }
    }
}
