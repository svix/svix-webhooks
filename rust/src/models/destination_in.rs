// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    azure_blob_storage_config_in::AzureBlobStorageConfigIn, big_query_config_in::BigQueryConfigIn,
    clickhouse_config_in::ClickhouseConfigIn, event_bridge_config_in::EventBridgeConfigIn,
    fifo_endpoint_config_in::FifoEndpointConfigIn,
    google_cloud_pub_sub_config_in::GoogleCloudPubSubConfigIn,
    google_cloud_storage_config_in::GoogleCloudStorageConfigIn,
    otel_tracing_config_in::OtelTracingConfigIn, postgres_config_in::PostgresConfigIn,
    rabbit_mq_config_in::RabbitMqConfigIn, redshift_config_in::RedshiftConfigIn,
    s3_config_in::S3ConfigIn, sink_status_in::SinkStatusIn, snowflake_config_in::SnowflakeConfigIn,
    sns_config_in::SnsConfigIn, sqs_config_in::SqsConfigIn,
};

/// The destination's type and type-specific configuration.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DestinationIn {
    /// An optional unique identifier for the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    /// Whether the destination will receive events.
    ///
    /// If the destination is `enabled`, events sent to the application will be
    /// dispatched to the destination in order.
    ///
    /// If the destination is `disabled`, events will not be dispatched until
    /// the destination is reenabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SinkStatusIn>,

    /// How many events will be batched in a request to the destination.
    #[serde(rename = "batchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<u16>,

    /// How long to wait before a batch of events is sent, if the `batchSize` is
    /// not reached.
    ///
    /// For example, with a `batchSize` of 100 and `maxWaitSecs` of 10, a
    /// request is sent after 10 seconds or 100 events, whichever comes first.
    ///
    /// Note that an empty batch is never sent to the destination.
    #[serde(rename = "maxWaitSecs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wait_secs: Option<u16>,

    /// A list of event types that filter which events are dispatched to the
    /// destination. An empty list (or null) will not filter out any events.
    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    /// A list of channels that filter which events are dispatched to the
    /// destination. An empty list (or null) will not filter out any events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,

    #[serde(flatten)]
    pub config: DestinationInConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum DestinationInConfig {
    #[serde(rename = "pollingEndpoint")]
    PollingEndpoint,
    #[serde(rename = "azureBlobStorage")]
    AzureBlobStorage(AzureBlobStorageConfigIn),
    #[serde(rename = "otelTracing")]
    OtelTracing(OtelTracingConfigIn),
    #[serde(rename = "fifoEndpoint")]
    FifoEndpoint(FifoEndpointConfigIn),
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
    #[serde(rename = "postgres")]
    Postgres(PostgresConfigIn),
}
