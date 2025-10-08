// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    azure_blob_storage_config::AzureBlobStorageConfig,
    google_cloud_storage_config::GoogleCloudStorageConfig, s3_config::S3Config,
    sink_http_config::SinkHttpConfig, sink_otel_v1_config::SinkOtelV1Config,
    sink_status_in::SinkStatusIn,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamSinkIn {
    /// How many events will be batched in a request to the Sink.
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

    /// A list of event types that filter which events are dispatched to the
    /// Sink. An empty list (or null) will not filter out any events.
    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    /// How long to wait before a batch of events is sent, if the `batchSize` is
    /// not reached.
    ///
    /// For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, we will
    /// send a request after 10 seconds or 100 events, whichever comes first.
    ///
    /// Note that we will never send an empty batch of events to the Sink.
    #[serde(rename = "maxWaitSecs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wait_secs: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    /// Whether the sink will receive events.
    ///
    /// If the sink is `enabled`, any events posted to the stream will be
    /// dispatched to the Sink in the same order that events were posted to the
    /// stream.
    ///
    /// If the sink is `disabled`, events will not be dispatched to the sink
    /// until the sink is reenabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SinkStatusIn>,

    /// An optional unique identifier for the sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    #[serde(flatten)]
    pub config: StreamSinkInConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum StreamSinkInConfig {
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
impl Default for StreamSinkInConfig {
    fn default() -> Self {
        Self::Poller
    }
}
