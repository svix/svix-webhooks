// this file is @generated
use serde::{Deserialize, Serialize};

use super::retention::Retention;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgNamespaceGetOut {
    pub name: String,

    pub retention: Retention,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl MsgNamespaceGetOut {
    pub fn new(
        name: String,
        retention: Retention,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            name,
            retention,
            created,
            updated,
        }
    }
}
