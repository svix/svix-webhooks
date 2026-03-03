// this file is @generated
use serde::{Deserialize, Serialize};

use super::{retention::Retention, storage_type::StorageType};

#[derive(Clone, Debug, Deserialize)]
pub struct MsgNamespaceCreateIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<Retention>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,
}

impl MsgNamespaceCreateIn {
    pub fn new() -> Self {
        Self {
            retention: None,
            storage_type: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgNamespaceCreateIn_ {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<Retention>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<StorageType>,
}
