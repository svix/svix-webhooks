// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnowflakePatchConfig {
    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,

    #[serde(rename = "accountIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<String>,

    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Database name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "dbName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,

    /// Schema name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "schemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,

    /// Table name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

impl SnowflakePatchConfig {
    pub fn new() -> Self {
        Self {
            private_key: None,
            account_identifier: None,
            user_id: None,
            db_name: None,
            schema_name: None,
            table_name: None,
        }
    }
}

impl Default for SnowflakePatchConfig {
    fn default() -> Self {
        Self::new()
    }
}
