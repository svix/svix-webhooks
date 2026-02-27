// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyGetNamespaceIn {
    pub name: String,
}

impl IdempotencyGetNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
