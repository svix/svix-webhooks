// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleUpsertOut {
    pub id: String,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl AdminRoleUpsertOut {
    pub fn new(id: String, created: jiff::Timestamp, updated: jiff::Timestamp) -> Self {
        Self {
            id,
            created,
            updated,
        }
    }
}
