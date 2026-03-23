// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenDeleteOut {
    pub success: bool,
}

impl AuthTokenDeleteOut {
    pub fn new(success: bool) -> Self {
        Self { success }
    }
}
