// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct OpenClawConfig {
    pub secret: String,
}

impl OpenClawConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
