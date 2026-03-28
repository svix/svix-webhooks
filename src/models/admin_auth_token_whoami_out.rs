// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenWhoamiOut {
    pub role: String,
}

impl AdminAuthTokenWhoamiOut {
    pub fn new(role: String) -> Self {
        Self { role }
    }
}
