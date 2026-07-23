// this file is @generated
use js_option::JsOption;
use serde::{Deserialize, Serialize};

use super::{
    amazon_s3_patch_config::AmazonS3PatchConfig,
    azure_blob_storage_patch_config::AzureBlobStoragePatchConfig,
    big_query_patch_config::BigQueryPatchConfig, clickhouse_patch_config::ClickhousePatchConfig,
    event_bridge_patch_config::EventBridgePatchConfig,
    google_cloud_pub_sub_patch_config::GoogleCloudPubSubPatchConfig,
    google_cloud_storage_patch_config::GoogleCloudStoragePatchConfig,
    http_patch_config::HttpPatchConfig, otel_tracing_patch_config::OtelTracingPatchConfig,
    rabbit_mq_patch_config::RabbitMqPatchConfig, redshift_patch_config::RedshiftPatchConfig,
    sink_status_in::SinkStatusIn, snowflake_patch_config::SnowflakePatchConfig,
    sns_patch_config::SnsPatchConfig, sqs_patch_config::SqsPatchConfig,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamSinkPatch {
    /// The StreamSink's UID.
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub uid: JsOption<String>,

    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub status: JsOption<SinkStatusIn>,

    #[serde(rename = "batchSize")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub batch_size: JsOption<u16>,

    #[serde(rename = "maxWaitSecs")]
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    pub max_wait_secs: JsOption<u16>,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

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
    #[serde(rename = "googleCloudPubSub")]
    GoogleCloudPubSub(GoogleCloudPubSubPatchConfig),
    #[serde(rename = "sqs")]
    Sqs(SqsPatchConfig),
    #[serde(rename = "sns")]
    Sns(SnsPatchConfig),
    #[serde(rename = "bigQuery")]
    BigQuery(BigQueryPatchConfig),
    #[serde(rename = "clickhouse")]
    Clickhouse(ClickhousePatchConfig),
    #[serde(rename = "eventBridge")]
    EventBridge(EventBridgePatchConfig),
    #[serde(rename = "snowflake")]
    Snowflake(SnowflakePatchConfig),
    #[serde(rename = "rabbitMq")]
    RabbitMq(RabbitMqPatchConfig),
    #[serde(rename = "redshift")]
    Redshift(RedshiftPatchConfig),
}
