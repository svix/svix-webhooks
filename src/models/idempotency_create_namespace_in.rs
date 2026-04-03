// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdempotencyCreateNamespaceIn {
    pub name: String,
}

impl IdempotencyCreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
