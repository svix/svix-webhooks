// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PortIoConfig {
    pub secret: String,
}

impl PortIoConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
