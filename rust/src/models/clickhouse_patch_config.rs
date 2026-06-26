// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ClickhousePatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ClickhousePatchConfig {
    pub fn new() -> Self {
        Self {
            database: None,
            password: None,
            table_name: None,
            url: None,
            username: None,
        }
    }
}
