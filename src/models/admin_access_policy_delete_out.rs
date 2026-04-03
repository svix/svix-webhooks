// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAccessPolicyDeleteOut {
    pub success: bool,
}

impl AdminAccessPolicyDeleteOut {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
