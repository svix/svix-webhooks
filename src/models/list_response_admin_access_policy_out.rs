// this file is @generated
use serde::{Deserialize, Serialize};

use super::admin_access_policy_out::AdminAccessPolicyOut;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListResponseAdminAccessPolicyOut {
    pub data: Vec<AdminAccessPolicyOut>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_iterator: Option<String>,

    pub done: bool,
}

impl ListResponseAdminAccessPolicyOut {
    pub fn new(data: Vec<AdminAccessPolicyOut>, done: bool) -> Self {
        Self {
            data,
            iterator: None,
            prev_iterator: None,
            done,
        }
    }

    pub fn with_iterator(mut self, value: impl Into<Option<String>>) -> Self {
        self.iterator = value.into();
        self
    }

    pub fn with_prev_iterator(mut self, value: impl Into<Option<String>>) -> Self {
        self.prev_iterator = value.into();
        self
    }
}
