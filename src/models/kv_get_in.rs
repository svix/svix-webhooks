// this file is @generated
use serde::{Deserialize, Serialize};

use super::consistency::Consistency;

#[derive(Clone, Debug, Deserialize)]
pub struct KvGetIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<Consistency>,
}

impl KvGetIn {
    pub fn new() -> Self {
        Self { consistency: None }
    }

    pub fn with_consistency(mut self, value: impl Into<Option<Consistency>>) -> Self {
        self.consistency = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct KvGetIn_ {
    pub key: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<Consistency>,
}
