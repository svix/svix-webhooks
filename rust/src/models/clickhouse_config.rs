// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ClickhouseConfig {
    /// The Clickhouse database to connect to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,

    /// Password to access Clickhouse
    pub password: String,

    /// The Clickhouse table to write to
    #[serde(rename = "tableName")]
    pub table_name: String,

    /// The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
    pub url: String,

    /// Username to access Clickhouse
    pub username: String,
}

impl ClickhouseConfig {
    pub fn new(password: String, table_name: String, url: String, username: String) -> Self {
        Self {
            database: None,
            password,
            table_name,
            url,
            username,
        }
    }
}
