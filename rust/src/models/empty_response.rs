// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EmptyResponse {}

impl EmptyResponse {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EmptyResponse {
    fn default() -> Self {
        Self::new()
    }
}
