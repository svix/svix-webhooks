// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AirwallexConfig {
    pub secret: String,
}

impl AirwallexConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
