// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ClickhouseConfig {
    /// The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
    pub url: String,

    /// Username to access Clickhouse
    pub username: String,

    /// Password to access Clickhouse
    pub password: String,

    /// The Clickhouse database to connect to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,

    /// The Clickhouse table to write to
    #[serde(rename = "tableName")]
    pub table_name: String,
}

impl ClickhouseConfig {
    pub fn new(url: String, username: String, password: String, table_name: String) -> Self {
        Self {
            url,
            username,
            password,
            database: None,
            table_name,
        }
    }
}
