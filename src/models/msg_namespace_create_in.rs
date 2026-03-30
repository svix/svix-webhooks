// this file is @generated
use serde::{Deserialize, Serialize};

use super::retention::Retention;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MsgNamespaceCreateIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<Retention>,
}

impl MsgNamespaceCreateIn {
    pub fn new() -> Self {
        Self { retention: None }
    }

    pub fn with_retention(mut self, value: impl Into<Option<Retention>>) -> Self {
        self.retention = value.into();
        self
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MsgNamespaceCreateIn_ {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<Retention>,
}
