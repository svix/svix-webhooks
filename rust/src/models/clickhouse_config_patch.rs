// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ClickhouseConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,

    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

impl ClickhouseConfigPatch {
    pub fn new() -> Self {
        Self {
            url: None,
            username: None,
            password: None,
            database: None,
            table_name: None,
        }
    }
}

impl Default for ClickhouseConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
