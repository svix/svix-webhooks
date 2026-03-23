// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenRotateOut {
    pub id: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,

    pub token: String,
}

impl AuthTokenRotateOut {
    pub fn new(
        id: String,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
        token: String,
    ) -> Self {
        Self {
            id,
            created,
            updated,
            token,
        }
    }
}
