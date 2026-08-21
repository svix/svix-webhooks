// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RedshiftConfigOut {
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    pub region: String,

    #[serde(rename = "clusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,

    #[serde(rename = "dbUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,

    #[serde(rename = "workgroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,

    /// Table name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}
