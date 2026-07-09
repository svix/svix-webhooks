// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SlackConfigOut {}

impl SlackConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for SlackConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
