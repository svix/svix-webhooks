// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PortIoConfigOut {}

impl PortIoConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PortIoConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
