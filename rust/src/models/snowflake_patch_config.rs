// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SnowflakePatchConfig {
    #[serde(rename = "accountIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<String>,

    /// Database name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "dbName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,

    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,

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

    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl SnowflakePatchConfig {
    pub fn new() -> Self {
        Self {
            account_identifier: None,
            db_name: None,
            private_key: None,
            schema_name: None,
            table_name: None,
            user_id: None,
        }
    }
}
