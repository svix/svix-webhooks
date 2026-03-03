// this file is @generated
use serde::{Deserialize, Serialize};

use super::{retention::Retention, storage_type::StorageType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgNamespaceCreateOut {
    pub name: String,

    pub retention: Retention,

    pub storage_type: StorageType,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl MsgNamespaceCreateOut {
    pub fn new(
        name: String,
        retention: Retention,
        storage_type: StorageType,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            name,
            retention,
            storage_type,
            created,
            updated,
        }
    }
}
