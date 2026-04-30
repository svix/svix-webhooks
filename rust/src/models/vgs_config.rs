// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VgsConfig {
    pub secret: String,
}

impl VgsConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
