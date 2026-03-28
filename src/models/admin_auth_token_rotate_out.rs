// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAuthTokenRotateOut {
    pub id: String,

    pub token: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl AdminAuthTokenRotateOut {
    pub fn new(
        id: String,
        token: String,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            id,
            token,
            created,
            updated,
        }
    }
}
