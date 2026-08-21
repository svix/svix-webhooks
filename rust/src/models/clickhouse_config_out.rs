// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ClickhouseConfigOut {
    pub url: String,

    pub username: String,

    pub database: String,

    #[serde(rename = "tableName")]
    pub table_name: String,
}
