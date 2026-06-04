// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration parameters for defining a Snowflake sink.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SnowflakeConfig {
    /// Snowflake account identifier, which includes both the organization and
    /// account IDs separated by a hyphen.
    #[serde(rename = "accountIdentifier")]
    pub account_identifier: String,

    /// Database name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "dbName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,

    /// PEM-encoded private key used for signing token-based requests to the
    /// Snowflake API.
    ///
    /// Beginning/end delimiters are not required.
    #[serde(rename = "privateKey")]
    pub private_key: String,

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

    /// The Snowflake user id.
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl SnowflakeConfig {
    pub fn new(account_identifier: String, private_key: String, user_id: String) -> Self {
        Self {
            account_identifier,
            db_name: None,
            private_key,
            schema_name: None,
            table_name: None,
            user_id,
        }
    }
}
