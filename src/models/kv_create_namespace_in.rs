// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KvCreateNamespaceIn {
    pub name: String,
}

impl KvCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
