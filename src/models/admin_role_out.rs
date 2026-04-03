// this file is @generated
use serde::{Deserialize, Serialize};

use super::access_rule::AccessRule;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdminRoleOut {
    pub id: String,

    pub description: String,

    pub rules: Vec<AccessRule>,

    pub policies: Vec<String>,

    pub context: std::collections::HashMap<String, String>,

    pub created: jiff::Timestamp,

    pub updated: jiff::Timestamp,
}

impl AdminRoleOut {
    pub fn new(
        id: String,
        description: String,
        rules: Vec<AccessRule>,
        policies: Vec<String>,
        context: std::collections::HashMap<String, String>,
        created: jiff::Timestamp,
        updated: jiff::Timestamp,
    ) -> Self {
        Self {
            id,
            description,
            rules,
            policies,
            context,
            created,
            updated,
        }
    }
}
