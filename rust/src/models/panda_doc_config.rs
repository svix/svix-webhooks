// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PandaDocConfig {
    pub secret: String,
}

impl PandaDocConfig {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}
