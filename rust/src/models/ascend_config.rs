// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AscendConfig {
    pub secret: String,
}

impl AscendConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
