// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GithubConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl GithubConfig {
    pub fn new() -> Self {
        Self { secret: None }
    }
}
