// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NangoConfigOut {}

impl NangoConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for NangoConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
