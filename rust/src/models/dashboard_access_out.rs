// this file is @generated
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DashboardAccessOut {
    pub token: String,

    pub url: String,
}

impl DashboardAccessOut {
    pub fn new(
        token: String,
        url: String,
    ) -> Self {
        Self {
            token,
            url,
        }
    }
}
