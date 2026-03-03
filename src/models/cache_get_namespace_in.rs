// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CacheGetNamespaceIn {
    pub name: String,
}

impl CacheGetNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
