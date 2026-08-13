// this file is @generated
#[allow(unused_imports)]
use js_option::JsOption;
use serde::{Deserialize, Serialize};

use super::{
    azure_blob_storage_config_patch::AzureBlobStorageConfigPatch,
    big_query_config_patch::BigQueryConfigPatch, clickhouse_config_patch::ClickhouseConfigPatch,
    event_bridge_config_patch::EventBridgeConfigPatch,
    google_cloud_pub_sub_config_patch::GoogleCloudPubSubConfigPatch,
    google_cloud_storage_config_patch::GoogleCloudStorageConfigPatch,
    otel_tracing_config_patch::OtelTracingConfigPatch, rabbit_mq_config_patch::RabbitMqConfigPatch,
    redshift_config_patch::RedshiftConfigPatch, s3_config_patch::S3ConfigPatch,
    sink_http_config_patch::SinkHttpConfigPatch, sink_status_in::SinkStatusIn,
    snowflake_config_patch::SnowflakeConfigPatch, sns_config_patch::SnsConfigPatch,
    sqs_config_patch::SqsConfigPatch,
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
    pub channels: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,

    #[serde(flatten)]
    pub config: StreamSinkPatchConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum StreamSinkPatchConfig {
    #[serde(rename = "poller")]
    Poller,
    #[serde(rename = "azureBlobStorage")]
    AzureBlobStorage(AzureBlobStorageConfigPatch),
    #[serde(rename = "otelTracing")]
    OtelTracing(OtelTracingConfigPatch),
    #[serde(rename = "http")]
    Http(SinkHttpConfigPatch),
    #[serde(rename = "amazonS3")]
    AmazonS3(S3ConfigPatch),
    #[serde(rename = "googleCloudStorage")]
    GoogleCloudStorage(GoogleCloudStorageConfigPatch),
    #[serde(rename = "googleCloudPubSub")]
    GoogleCloudPubSub(GoogleCloudPubSubConfigPatch),
    #[serde(rename = "sqs")]
    Sqs(SqsConfigPatch),
    #[serde(rename = "sns")]
    Sns(SnsConfigPatch),
    #[serde(rename = "bigQuery")]
    BigQuery(BigQueryConfigPatch),
    #[serde(rename = "clickhouse")]
    Clickhouse(ClickhouseConfigPatch),
    #[serde(rename = "eventBridge")]
    EventBridge(EventBridgeConfigPatch),
    #[serde(rename = "snowflake")]
    Snowflake(SnowflakeConfigPatch),
    #[serde(rename = "rabbitMq")]
    RabbitMq(RabbitMqConfigPatch),
    #[serde(rename = "redshift")]
    Redshift(RedshiftConfigPatch),
}
