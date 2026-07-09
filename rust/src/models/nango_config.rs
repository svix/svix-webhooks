// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NangoConfig {
    pub secret: String,
}

impl NangoConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
