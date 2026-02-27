// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetIn {
    pub key: String,
}

impl KvGetIn {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
