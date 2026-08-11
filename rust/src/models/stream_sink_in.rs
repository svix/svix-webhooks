// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    azure_blob_storage_config_in::AzureBlobStorageConfigIn, big_query_config_in::BigQueryConfigIn,
    clickhouse_config_in::ClickhouseConfigIn, event_bridge_config_in::EventBridgeConfigIn,
    google_cloud_pub_sub_config_in::GoogleCloudPubSubConfigIn,
    google_cloud_storage_config_in::GoogleCloudStorageConfigIn,
    rabbit_mq_config_in::RabbitMqConfigIn, redshift_config_in::RedshiftConfigIn,
    s3_config_in::S3ConfigIn, sink_http_config_in::SinkHttpConfigIn,
    sink_otel_tracing_config_in::SinkOtelTracingConfigIn, sink_status_in::SinkStatusIn,
    snowflake_config_in::SnowflakeConfigIn, sns_config_in::SnsConfigIn, sqs_config_in::SqsConfigIn,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamSinkIn {
    /// An optional unique identifier for the sink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

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

    /// How many events will be batched in a request to the Sink.
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

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

    /// A list of event types that filter which events are dispatched to the
    /// Sink. An empty list (or null) will not filter out any events.
    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,

    #[serde(flatten)]
    pub config: StreamSinkInConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum StreamSinkInConfig {
    #[serde(rename = "poller")]
    Poller,
    #[serde(rename = "azureBlobStorage")]
    AzureBlobStorage(AzureBlobStorageConfigIn),
    #[serde(rename = "otelTracing")]
    OtelTracing(SinkOtelTracingConfigIn),
    #[serde(rename = "http")]
    Http(SinkHttpConfigIn),
    #[serde(rename = "amazonS3")]
    AmazonS3(S3ConfigIn),
    #[serde(rename = "googleCloudStorage")]
    GoogleCloudStorage(GoogleCloudStorageConfigIn),
    #[serde(rename = "googleCloudPubSub")]
    GoogleCloudPubSub(GoogleCloudPubSubConfigIn),
    #[serde(rename = "sqs")]
    Sqs(SqsConfigIn),
    #[serde(rename = "sns")]
    Sns(SnsConfigIn),
    #[serde(rename = "bigQuery")]
    BigQuery(BigQueryConfigIn),
    #[serde(rename = "clickhouse")]
    Clickhouse(ClickhouseConfigIn),
    #[serde(rename = "eventBridge")]
    EventBridge(EventBridgeConfigIn),
    #[serde(rename = "snowflake")]
    Snowflake(SnowflakeConfigIn),
    #[serde(rename = "rabbitMq")]
    RabbitMq(RabbitMqConfigIn),
    #[serde(rename = "redshift")]
    Redshift(RedshiftConfigIn),
}
