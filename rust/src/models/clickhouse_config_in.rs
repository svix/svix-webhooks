// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ClickhouseConfigIn {
    /// The HTTP URL of the ClickHouse server (e.g. `https://my_clickhouse:8443`).
    pub url: String,

    /// Username to access Clickhouse.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Password to access Clickhouse.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// The Clickhouse database to connect to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,

    /// The Clickhouse table to write to.
    #[serde(rename = "tableName")]
    pub table_name: String,
}

impl ClickhouseConfigIn {
    pub fn new(url: String, table_name: String) -> Self {
        Self {
            url,
            username: None,
            password: None,
            database: None,
            table_name,
        }
    }
}
