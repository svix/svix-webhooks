// this file is @generated
use serde::{Deserialize, Serialize};

use super::bulk_expunge_status::BulkExpungeStatus;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BulkExpungeContentsOut {
    /// Results of expunging (by ID)
    pub results: std::collections::BTreeMap<String, BulkExpungeStatus>,
}
