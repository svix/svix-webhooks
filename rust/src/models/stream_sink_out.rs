// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    azure_blob_storage_config::AzureBlobStorageConfig,
    google_cloud_storage_config::GoogleCloudStorageConfig, s3_config::S3Config,
    sink_http_config::SinkHttpConfig, sink_otel_v1_config::SinkOtelV1Config,
    sink_status::SinkStatus,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamSinkOut {
    #[serde(rename = "batchSize")]
    pub batch_size: i32,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "currentIterator")]
    pub current_iterator: String,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    /// The sink's ID.
    pub id: String,

    #[serde(rename = "maxWaitSecs")]
    pub max_wait_secs: i32,

    pub metadata: std::collections::HashMap<String, String>,

    #[serde(rename = "nextRetryAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_at: Option<String>,

    pub status: SinkStatus,

    /// The sink's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(flatten)]
    pub config: StreamSinkOutConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum StreamSinkOutConfig {
    #[serde(rename = "poller")]
    Poller,
    #[serde(rename = "azureBlobStorage")]
    AzureBlobStorage(AzureBlobStorageConfig),
    #[serde(rename = "otelTracing")]
    OtelTracing(SinkOtelV1Config),
    #[serde(rename = "http")]
    Http(SinkHttpConfig),
    #[serde(rename = "amazonS3")]
    AmazonS3(S3Config),
    #[serde(rename = "googleCloudStorage")]
    GoogleCloudStorage(GoogleCloudStorageConfig),
}

#[allow(clippy::derivable_impls)]
impl Default for StreamSinkOutConfig {
    fn default() -> Self {
        Self::Poller
    }
}
