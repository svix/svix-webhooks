// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetIn {
    pub key: String,
}

impl CacheGetIn {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
