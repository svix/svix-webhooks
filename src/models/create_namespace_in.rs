// this file is @generated
use serde::{Deserialize, Serialize};

use super::{retention::Retention, storage_type::StorageType};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateNamespaceIn {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<Retention>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,
}

impl CreateNamespaceIn {
    pub fn new(name: String) -> Self {
        Self {
            name,
            retention: None,
            storage_type: None,
        }
    }
}
