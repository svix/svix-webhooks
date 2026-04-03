// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleGetIn {
    pub id: String,
}

impl AdminRoleGetIn {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
