// this file is @generated
use js_option::JsOption;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    amazon_s3_patch_config::AmazonS3PatchConfig,
    azure_blob_storage_patch_config::AzureBlobStoragePatchConfig,
    google_cloud_storage_patch_config::GoogleCloudStoragePatchConfig,
    http_patch_config::HttpPatchConfig,
    otel_tracing_patch_config::OtelTracingPatchConfig,
    sink_status_in::SinkStatusIn,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StreamSinkPatch {
    #[serde(rename = "batchSize")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub batch_size: JsOption<u16>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    #[serde(rename = "maxWaitSecs")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub max_wait_secs: JsOption<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub status: JsOption<SinkStatusIn>,

    /// The StreamSink's UID.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uid: JsOption<String>,

    #[serde(flatten)]
    pub config: StreamSinkPatchConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum StreamSinkPatchConfig {
    #[serde(rename = "poller")]
    Poller,
    #[serde(rename = "azureBlobStorage")]
    AzureBlobStorage(AzureBlobStoragePatchConfig),
    #[serde(rename = "otelTracing")]
    OtelTracing(OtelTracingPatchConfig),
    #[serde(rename = "http")]
    Http(HttpPatchConfig),
    #[serde(rename = "amazonS3")]
    AmazonS3(AmazonS3PatchConfig),
    #[serde(rename = "googleCloudStorage")]
    GoogleCloudStorage(GoogleCloudStoragePatchConfig),
}

#[allow(clippy::derivable_impls)]
impl Default for StreamSinkPatchConfig {
    fn default() -> Self {
        Self::Poller
    }
}
