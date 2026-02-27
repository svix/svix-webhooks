// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvDeleteIn {
    pub key: String,
}

impl KvDeleteIn {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
