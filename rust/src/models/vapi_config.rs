// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VapiConfig {
    pub secret: String,
}

impl VapiConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
