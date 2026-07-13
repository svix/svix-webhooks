// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AppPortalAccessOut {
    pub url: String,

    pub token: String,
}

impl AppPortalAccessOut {
    pub fn new(url: String, token: String) -> Self {
        Self { url, token }
    }
}
