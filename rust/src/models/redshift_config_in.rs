// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration parameters for defining a Redshift sink.
///
/// For provisioned clusters, set `cluster_identifier` and `db_user`. For
/// Redshift Serverless, set `workgroup_name`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RedshiftConfigIn {
    /// Access key ID.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "accessKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    /// Secret access key.
    ///
    /// Currently a required field, but marked as optional because we may add
    /// different authentication in the future.
    #[serde(rename = "secretAccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,

    /// The region of the Redshift DB.
    ///
    /// Currently a required field, but marked as optional because we may infer
    /// it from other fields in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

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

impl RedshiftConfigIn {
    pub fn new() -> Self {
        Self {
            access_key_id: None,
            secret_access_key: None,
            region: None,
            cluster_identifier: None,
            db_user: None,
            workgroup_name: None,
            db_name: None,
            schema_name: None,
            table_name: None,
        }
    }
}

impl Default for RedshiftConfigIn {
    fn default() -> Self {
        Self::new()
    }
}
