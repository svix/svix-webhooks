// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenRotateIn {
    pub id: String,
}

impl AdminAuthTokenRotateIn {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
