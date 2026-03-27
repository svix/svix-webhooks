// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenDeleteOut {
    pub success: bool,
}

impl AdminAuthTokenDeleteOut {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
