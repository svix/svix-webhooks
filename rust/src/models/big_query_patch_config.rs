// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BigQueryPatchConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,

    #[serde(rename = "datasetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<String>,

    #[serde(rename = "projectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    #[serde(rename = "tableId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
}

impl BigQueryPatchConfig {
    pub fn new() -> Self {
        Self {
            credentials: None,
            dataset_id: None,
            project_id: None,
            table_id: None,
        }
    }
}
