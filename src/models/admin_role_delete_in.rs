// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleDeleteIn {
    pub id: String,
}

impl AdminRoleDeleteIn {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
