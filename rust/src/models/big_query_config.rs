// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a Google Cloud BigQuery sink.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BigQueryConfig {
    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "datasetId")]
    pub dataset_id: String,

    #[serde(rename = "tableId")]
    pub table_id: String,

    /// Google Cloud Credentials JSON Object as a string.
    pub credentials: String,
}

impl BigQueryConfig {
    pub fn new(
        project_id: String,
        dataset_id: String,
        table_id: String,
        credentials: String,
    ) -> Self {
        Self {
            project_id,
            dataset_id,
            table_id,
            credentials,
        }
    }
}
