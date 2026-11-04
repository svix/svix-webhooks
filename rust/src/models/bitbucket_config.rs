// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BitbucketConfig {
    pub secret: String,
}

impl BitbucketConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
