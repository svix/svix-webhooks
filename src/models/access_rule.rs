// this file is @generated
use serde::{Deserialize, Serialize};

use super::access_rule_effect::AccessRuleEffect;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessRule {
    pub effect: AccessRuleEffect,

    pub resource: String,

    pub actions: Vec<String>,
}

impl AccessRule {
    pub fn new(effect: AccessRuleEffect, resource: String, actions: Vec<String>) -> Self {
        Self {
            effect,
            resource,
            actions,
        }
    }
}
