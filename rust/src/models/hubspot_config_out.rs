// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct HubspotConfigOut {}

impl HubspotConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for HubspotConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
