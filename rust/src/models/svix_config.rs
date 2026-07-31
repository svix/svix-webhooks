// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SvixConfig {
    pub secret: String,
}

impl SvixConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
