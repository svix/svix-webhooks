use serde::Deserialize;
use std::net::SocketAddr;
use svix_webhook_bridge_plugin_queue::config::{
    into_receiver_output, QueueConsumerConfig, ReceiverOutputOpts as QueueOutOpts,
};
use svix_webhook_bridge_types::{ReceiverInputOpts, ReceiverOutput, SenderInput};
use tracing::Level;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub senders: Vec<SenderConfig>,
    #[serde(default)]
    pub receivers: Vec<ReceiverConfig>,
    /// The log level to run the service with. Supported: info, debug, trace
    #[serde(default)]
    pub log_level: LogLevel,
    /// The log format that all output will follow. Supported: default, json
    #[serde(default)]
    pub log_format: LogFormat,
    /// OpenTelemetry exporter settings
    #[serde(default)]
    pub opentelemetry: Option<OtelExporterConfig>,
    #[serde(default = "default_http_listen_address")]
    pub http_listen_address: SocketAddr,
}

fn default_http_listen_address() -> SocketAddr {
    "0.0.0.0:5000".parse().expect("default http listen address")
}

#[derive(Deserialize)]
pub struct OtelExporterConfig {
    /// The OpenTelemetry service name to use
    pub service_name: Option<String>,
    /// The OpenTelemetry address to send events to if given.
    pub address: String,
    /// The ratio at which to sample spans when sending to OpenTelemetry. When not given it defaults
    /// to always sending. If the OpenTelemetry address is not set, this will do nothing.
    pub sample_ratio: Option<f64>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    #[default]
    Info,
    Debug,
    Trace,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            Self::Info => Level::INFO,
            Self::Debug => Level::DEBUG,
            Self::Trace => Level::TRACE,
        }
        .to_string()
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Default,
    Json,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SenderConfig {
    #[cfg(any(
        feature = "gcp-pubsub",
        feature = "rabbitmq",
        feature = "redis",
        feature = "sqs"
    ))]
    QueueConsumer(QueueConsumerConfig),
}

impl TryInto<Box<dyn SenderInput>> for SenderConfig {
    type Error = &'static str;

    fn try_into(self) -> Result<Box<dyn SenderInput>, Self::Error> {
        match self {
            #[cfg(any(
                feature = "gcp-pubsub",
                feature = "rabbitmq",
                feature = "redis",
                feature = "sqs"
            ))]
            SenderConfig::QueueConsumer(backend) => backend.try_into(),
        }
    }
}

#[derive(Deserialize)]
pub struct ReceiverConfig {
    pub name: String,
    pub input: ReceiverInputOpts,
    #[serde(default)]
    pub transformation: Option<String>,
    pub output: ReceiverOut,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ReceiverOut {
    #[cfg(any(
        feature = "gcp-pubsub",
        feature = "rabbitmq",
        feature = "redis",
        feature = "sqs"
    ))]
    QueueProducer(QueueOutOpts),
}

impl ReceiverConfig {
    pub async fn into_receiver_output(self) -> std::io::Result<Box<dyn ReceiverOutput>> {
        match self.output {
            ReceiverOut::QueueProducer(x) => into_receiver_output(self.name.clone(), x)
                .await
                .map_err(Into::into),
        }
    }
}

#[cfg(test)]
mod tests;
