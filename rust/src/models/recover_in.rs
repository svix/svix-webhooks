// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RecoverIn {
    pub since: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<chrono::DateTime<chrono::Utc>>,
}

impl RecoverIn {
    pub fn new(since: chrono::DateTime<chrono::Utc>) -> Self {
        Self { since, until: None }
    }
}
