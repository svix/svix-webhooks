// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration parameters for defining a Snowflake sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SnowflakeConfigIn {
    /// PEM-encoded private key used for signing token-based requests to the
    /// Snowflake API.
    ///
    /// Beginning/end delimiters are not required.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,

    /// Snowflake account identifier, which includes both the organization and
    /// account IDs separated by a hyphen.
    #[serde(rename = "accountIdentifier")]
    pub account_identifier: String,

    /// The Snowflake user id.
    #[serde(rename = "userId")]
    pub user_id: String,

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

impl SnowflakeConfigIn {
    pub fn new(account_identifier: String, user_id: String) -> Self {
        Self {
            private_key: None,
            account_identifier,
            user_id,
            db_name: None,
            schema_name: None,
            table_name: None,
        }
    }
}
