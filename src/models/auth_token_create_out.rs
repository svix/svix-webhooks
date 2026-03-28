// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthTokenCreateOut {
    pub id: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,

    pub token: String,
}

impl AuthTokenCreateOut {
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
