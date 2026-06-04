// this file is @generated
use serde::{Deserialize, Serialize};

/// Configuration for a Google Cloud BigQuery sink.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BigQueryConfig {
    /// Google Cloud Credentials JSON Object as a string.
    pub credentials: String,

    #[serde(rename = "datasetId")]
    pub dataset_id: String,

    #[serde(rename = "projectId")]
    pub project_id: String,

    #[serde(rename = "tableId")]
    pub table_id: String,
}

impl BigQueryConfig {
    pub fn new(
        credentials: String,
        dataset_id: String,
        project_id: String,
        table_id: String,
    ) -> Self {
        Self {
            credentials,
            dataset_id,
            project_id,
            table_id,
        }
    }
}
