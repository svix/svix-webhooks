// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    azure_blob_storage_config_out::AzureBlobStorageConfigOut,
    big_query_config_out::BigQueryConfigOut, clickhouse_config_out::ClickhouseConfigOut,
    event_bridge_config_out::EventBridgeConfigOut,
    google_cloud_pub_sub_config_out::GoogleCloudPubSubConfigOut,
    google_cloud_storage_config_out::GoogleCloudStorageConfigOut,
    otel_tracing_config_out::OtelTracingConfigOut, postgres_config_out::PostgresConfigOut,
    rabbit_mq_config_out::RabbitMqConfigOut, redshift_config_out::RedshiftConfigOut,
    s3_config_out::S3ConfigOut, sink_http_config_out::SinkHttpConfigOut, sink_status::SinkStatus,
    snowflake_config_out::SnowflakeConfigOut, sns_config_out::SnsConfigOut,
    sqs_config_out::SqsConfigOut,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StreamSinkOut {
    /// The sink's ID.
    pub id: String,

    /// The sink's UID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,

    pub status: SinkStatus,

    #[serde(rename = "currentIterator")]
    pub current_iterator: String,

    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,

    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "batchSize")]
    pub batch_size: i32,

    #[serde(rename = "maxWaitSecs")]
    pub max_wait_secs: i32,

    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,

    #[serde(rename = "nextRetryAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_at: Option<chrono::DateTime<chrono::Utc>>,

    pub metadata: std::collections::BTreeMap<String, String>,

    #[serde(flatten)]
    pub config: StreamSinkOutConfig,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", content = "config")]
pub enum StreamSinkOutConfig {
    #[serde(rename = "poller")]
    Poller,
    #[serde(rename = "azureBlobStorage")]
    AzureBlobStorage(AzureBlobStorageConfigOut),
    #[serde(rename = "otelTracing")]
    OtelTracing(OtelTracingConfigOut),
    #[serde(rename = "http")]
    Http(SinkHttpConfigOut),
    #[serde(rename = "amazonS3")]
    AmazonS3(S3ConfigOut),
    #[serde(rename = "snowflake")]
    Snowflake(SnowflakeConfigOut),
    #[serde(rename = "googleCloudStorage")]
    GoogleCloudStorage(GoogleCloudStorageConfigOut),
    #[serde(rename = "googleCloudPubSub")]
    GoogleCloudPubSub(GoogleCloudPubSubConfigOut),
    #[serde(rename = "redshift")]
    Redshift(RedshiftConfigOut),
    #[serde(rename = "bigQuery")]
    BigQuery(BigQueryConfigOut),
    #[serde(rename = "clickhouse")]
    Clickhouse(ClickhouseConfigOut),
    #[serde(rename = "rabbitMq")]
    RabbitMq(RabbitMqConfigOut),
    #[serde(rename = "sqs")]
    Sqs(SqsConfigOut),
    #[serde(rename = "eventBridge")]
    EventBridge(EventBridgeConfigOut),
    #[serde(rename = "sns")]
    Sns(SnsConfigOut),
    #[serde(rename = "postgres")]
    Postgres(PostgresConfigOut),
}
