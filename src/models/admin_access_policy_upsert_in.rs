// this file is @generated
use serde::{Deserialize, Serialize};

use super::access_rule::AccessRule;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminAccessPolicyUpsertIn {
    pub id: String,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AccessRule>>,
}

impl AdminAccessPolicyUpsertIn {
    pub fn new(id: String, description: String) -> Self {
        Self {
            id,
            description,
            rules: None,
        }
    }

    pub fn with_rules(mut self, value: impl Into<Option<Vec<AccessRule>>>) -> Self {
        self.rules = value.into();
        self
    }
}
