// this file is @generated
use serde::{Deserialize, Serialize};

use super::access_rule::AccessRule;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleUpsertIn {
    pub id: String,

    pub description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<AccessRule>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
}

impl AdminRoleUpsertIn {
    pub fn new(id: String, description: String) -> Self {
        Self {
            id,
            description,
            rules: None,
            policies: None,
            context: None,
        }
    }

    pub fn with_rules(mut self, value: impl Into<Option<Vec<AccessRule>>>) -> Self {
        self.rules = value.into();
        self
    }

    pub fn with_policies(mut self, value: impl Into<Option<Vec<String>>>) -> Self {
        self.policies = value.into();
        self
    }

    pub fn with_context(
        mut self,
        value: impl Into<Option<std::collections::HashMap<String, String>>>,
    ) -> Self {
        self.context = value.into();
        self
    }
}
