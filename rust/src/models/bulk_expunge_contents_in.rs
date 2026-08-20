// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BulkExpungeContentsIn {
    /// Message ID or UID to delete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<std::collections::BTreeSet<String>>,
}

impl BulkExpungeContentsIn {
    pub fn new() -> Self {
        Self { ids: None }
    }
}

impl Default for BulkExpungeContentsIn {
    fn default() -> Self {
        Self::new()
    }
}
