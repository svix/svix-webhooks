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

    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,

    pub region: String,

    /// Required for provisioned clusters.
    #[serde(rename = "clusterIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,

    /// Required for provisioned clusters.
    #[serde(rename = "dbUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,

    /// Required for Redshift Serverless.
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

impl RedshiftConfig {
    pub fn new(access_key_id: String, secret_access_key: String, region: String) -> Self {
        Self {
            access_key_id,
            secret_access_key,
            region,
            cluster_identifier: None,
            db_user: None,
            workgroup_name: None,
            db_name: None,
            schema_name: None,
            table_name: None,
        }
    }
}
