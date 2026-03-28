// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenDeleteIn {
    pub id: String,
}

impl AdminAuthTokenDeleteIn {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
