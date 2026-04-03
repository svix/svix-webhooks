// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAccessPolicyGetIn {
    pub id: String,
}

impl AdminAccessPolicyGetIn {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
