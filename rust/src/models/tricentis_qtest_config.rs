// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TricentisQtestConfig {
    pub secret: String,
}

impl TricentisQtestConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
