// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAccessPolicyDeleteIn {
    pub id: String,
}

impl AdminAccessPolicyDeleteIn {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
