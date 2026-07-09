// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration parameters for defining a Redshift sink.
///
/// For provisioned clusters, set `cluster_identifier` and `db_user`. For
/// Redshift Serverless, set `workgroup_name`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RedshiftConfig {
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,

    /// Required for provisioned clusters.
    #[serde(rename = "clusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,

    /// Database name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "dbName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,

    /// Required for provisioned clusters.
    #[serde(rename = "dbUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,

    pub region: String,

    /// Schema name.
    ///
    /// Only used if not using transformations.
    #[serde(rename = "schemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    /// Table name.
    ///
    /// Only required if not using transformations.
    #[serde(rename = "tableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,

    /// Required for Redshift Serverless.
    #[serde(rename = "workgroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

impl RedshiftConfig {
    pub fn new(access_key_id: String, region: String, secret_access_key: String) -> Self {
        Self {
            access_key_id,
            cluster_identifier: None,
            db_name: None,
            db_user: None,
            region,
            schema_name: None,
            secret_access_key,
            table_name: None,
            workgroup_name: None,
        }
    }
}
