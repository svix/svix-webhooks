// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PostgresConfigPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

    #[serde(rename = "sslRootCert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_root_cert: Option<String>,
}

impl PostgresConfigPatch {
    pub fn new() -> Self {
        Self {
            url: None,
            password: None,
            table_name: None,
            ssl_root_cert: None,
        }
    }
}

impl Default for PostgresConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
