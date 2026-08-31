// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PostgresConfigOut {
    pub url: String,

    #[serde(rename = "tableName")]
    pub table_name: String,

    #[serde(rename = "sslRootCert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_root_cert: Option<String>,
}
