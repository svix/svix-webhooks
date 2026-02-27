// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvGetNamespaceIn {
    pub name: String,
}

impl KvGetNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
