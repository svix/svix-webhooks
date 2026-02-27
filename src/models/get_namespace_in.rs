// this file is @generated
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetNamespaceIn {
    pub name: String,
}

impl GetNamespaceIn {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
