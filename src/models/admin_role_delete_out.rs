// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleDeleteOut {
    pub success: bool,
}

impl AdminRoleDeleteOut {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
