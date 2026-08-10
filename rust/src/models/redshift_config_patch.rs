// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RedshiftConfigPatch {
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Database name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "dbName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,

    /// Schema name.
    ///
    /// Only used if not using transformations.
    #[serde(rename = "schemaName")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub schema_name: JsOption<String>,

    /// Table name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

impl RedshiftConfigPatch {
    pub fn new() -> Self {
        Self {
            access_key_id: None,
            secret_access_key: None,
            region: None,
            db_name: None,
            schema_name: JsOption::Undefined,
            table_name: None,
        }
    }
}

impl Default for RedshiftConfigPatch {
    fn default() -> Self {
        Self::new()
    }
}
