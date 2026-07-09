// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TailscaleConfigOut {}

impl TailscaleConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TailscaleConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
