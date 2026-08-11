// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BigQueryConfigOut {
    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "datasetId")]
    pub dataset_id: String,

    #[serde(rename = "tableId")]
    pub table_id: String,
}
