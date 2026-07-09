// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VapiConfigOut {}

impl VapiConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for VapiConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
