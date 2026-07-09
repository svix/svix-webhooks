// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GithubConfigOut {}

impl GithubConfigOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for GithubConfigOut {
    fn default() -> Self {
        Self::new()
    }
}
