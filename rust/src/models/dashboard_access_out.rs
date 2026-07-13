// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DashboardAccessOut {
    pub url: String,

    pub token: String,
}

impl DashboardAccessOut {
    pub fn new(url: String, token: String) -> Self {
        Self { url, token }
    }
}
