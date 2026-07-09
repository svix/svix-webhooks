// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StripeConfigOut {}

impl StripeConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StripeConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
