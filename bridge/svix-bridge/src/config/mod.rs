use std::{
    borrow::Cow, collections::HashMap, convert::Infallible, fmt, io::Error, net::SocketAddr,
    num::NonZeroUsize,
};

use anyhow::anyhow;
use serde::{Deserialize, Deserializer};
use shellexpand::LookupError;
#[cfg(feature = "kafka")]
use svix_bridge_plugin_kafka::{KafkaInputOpts, KafkaOutputOpts};
use svix_bridge_plugin_queue::config::{QueueInputOpts, QueueOutputOpts};
use svix_bridge_types::{
    svix::api::Svix, ReceiverInputOpts, ReceiverOutput, SenderInput, SenderOutputOpts, SvixOptions,
    TransformationConfig,
};
use tracing::Level;

use crate::http_output::HttpOutputOpts;

pub enum EitherReceiver {
    Webhook(WebhookReceiverConfig),
    Poller(PollerReceiverConfig),
}

impl<'de> Deserialize<'de> for EitherReceiver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;
        let input_type = value
            .as_mapping()
            .and_then(|m| m.get("input"))
            .and_then(|v| v.as_mapping())
            .and_then(|m| m.get("type"))
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| {
                serde::de::Error::custom(
                    "missing field 'type' in receiver 'input', expected one of: webhook, svix-webhook, svix-message-poller",
                )
            })?
            .to_owned();
        match input_type.as_str() {
            "webhook" | "svix-webhook" => serde_yaml::from_value(value)
                .map(EitherReceiver::Webhook)
                .map_err(serde::de::Error::custom),
            "svix-message-poller" => serde_yaml::from_value(value)
                .map(EitherReceiver::Poller)
                .map_err(serde::de::Error::custom),
            other => Err(serde::de::Error::custom(format!(
                "unknown receiver input type '{other}', expected one of: webhook, svix-webhook, svix-message-poller"
            ))),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Config for reading messages from plugins and forwarding to Svix.
    #[serde(default)]
    pub senders: Vec<WebhookSenderConfig>,
    /// Config for receiving webhooks and forwarding them to plugins.
    #[serde(default)]
    pub receivers: Vec<EitherReceiver>,
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
    #[serde(default = "default_transformation_worker_count")]
    pub transformation_worker_count: NonZeroUsize,
}

impl Config {
    /// Build a Config from yaml source.
    /// Optionally accepts a map to perform variable substitution with.
    pub fn from_src(
        raw_src: &str,
        vars: Option<&HashMap<String, String>>,
    ) -> std::io::Result<Self> {
        let src = if let Some(vars) = vars {
            let context = |key: &str| -> Result<Option<Cow<'_, str>>, LookupError<Infallible>> {
                Ok(vars.get(key).map(Cow::from))
            };
            shellexpand::env_with_context(raw_src, context).map_err(|e: LookupError<_>| {
                Error::other(format!("Variable substitution failed: {e}"))
            })?
        } else {
            Cow::Borrowed(raw_src)
        };
        let cfg: Self = serde_yaml::from_str(&src)
            .map_err(|e| Error::other(format!("Failed to parse config: {e}")))?;

        for sc in &cfg.senders {
            if let Some(tc) = sc.transformation() {
                crate::runtime::validate_script(tc.source().as_str()).map_err(|e| {
                    Error::other(format!(
                        "failed to parse transformation for sender `{}`: {e:?}",
                        sc.name(),
                    ))
                })?;
            }
        }

        for (name, tc) in cfg.receivers.iter().filter_map(|either| match either {
            EitherReceiver::Webhook(receiver) => receiver
                .transformation
                .as_ref()
                .map(|tc| (&receiver.name, tc)),
            EitherReceiver::Poller(receiver) => receiver
                .transformation
                .as_ref()
                .map(|tc| (&receiver.name, tc)),
        }) {
            crate::runtime::validate_script(tc.source().as_str()).map_err(|e| {
                Error::other(format!(
                    "failed to parse transformation for receiver `{name}`: {e:?}"
                ))
            })?;
        }

        Ok(cfg)
    }
}

fn default_http_listen_address() -> SocketAddr {
    "0.0.0.0:5000".parse().expect("default http listen address")
}

fn default_transformation_worker_count() -> NonZeroUsize {
    NonZeroUsize::new(4).expect("4 is greater than 0")
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

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => Level::INFO,
            Self::Debug => Level::DEBUG,
            Self::Trace => Level::TRACE,
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Default,
    Json,
}

/// Config for reading messages from plugins and forwarding to Svix.
#[derive(Deserialize)]
pub struct WebhookSenderConfig {
    pub name: String,
    pub input: SenderInputOpts,
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
    pub output: SenderOutputOpts,
}

pub enum SenderInputOpts {
    #[cfg(feature = "kafka")]
    Kafka(KafkaInputOpts),
    Queue(QueueInputOpts),
}

impl<'de> Deserialize<'de> for SenderInputOpts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;
        let type_val = value
            .as_mapping()
            .and_then(|m| m.get("type"))
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| {
                let valid = if cfg!(feature = "kafka") {
                    "kafka, rabbitmq, redis, sqs, gcp-pubsub"
                } else {
                    "rabbitmq, redis, sqs, gcp-pubsub"
                };
                serde::de::Error::custom(format!(
                    "missing field 'type' in sender input, expected one of: {valid}"
                ))
            })?
            .to_owned();

        match type_val.as_str() {
            #[cfg(feature = "kafka")]
            "kafka" => serde_yaml::from_value(value)
                .map(SenderInputOpts::Kafka)
                .map_err(serde::de::Error::custom),
            "rabbitmq" | "redis" | "sqs" | "gcp-pubsub" => serde_yaml::from_value(value)
                .map(SenderInputOpts::Queue)
                .map_err(serde::de::Error::custom),
            other => {
                let valid = if cfg!(feature = "kafka") {
                    "kafka, rabbitmq, redis, sqs, gcp-pubsub"
                } else {
                    "rabbitmq, redis, sqs, gcp-pubsub"
                };
                Err(serde::de::Error::custom(format!(
                    "unknown sender input type '{other}', expected one of {valid}"
                )))
            }
        }
    }
}

impl WebhookSenderConfig {
    pub fn into_sender_input(self) -> anyhow::Result<Box<dyn SenderInput>> {
        Ok(match self.input {
            #[cfg(feature = "kafka")]
            SenderInputOpts::Kafka(input_opts) => svix_bridge_plugin_kafka::into_sender_input(
                self.name,
                input_opts,
                self.transformation,
                self.output,
            )?,
            SenderInputOpts::Queue(input_opts) => svix_bridge_plugin_queue::into_sender_input(
                self.name,
                input_opts,
                self.transformation,
                self.output,
            )
            .map_err(|e| anyhow!("{e}"))?,
        })
    }
}

impl WebhookSenderConfig {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn transformation(&self) -> Option<&TransformationConfig> {
        self.transformation.as_ref()
    }
}

impl TryFrom<WebhookSenderConfig> for Box<dyn SenderInput> {
    type Error = anyhow::Error;

    fn try_from(value: WebhookSenderConfig) -> Result<Self, Self::Error> {
        value.into_sender_input()
    }
}

/// Config for receiving webhooks and forwarding them to plugins.
#[derive(Deserialize)]
pub struct WebhookReceiverConfig {
    pub name: String,
    pub input: ReceiverInputOpts,
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
    pub output: ReceiverOutputOpts,
}

#[allow(clippy::large_enum_variant)] // we're talking a couple hundred bytes only
pub enum ReceiverOutputOpts {
    Http(HttpOutputOpts),
    #[cfg(feature = "kafka")]
    Kafka(KafkaOutputOpts),
    Queue(QueueOutputOpts),
}

impl<'de> Deserialize<'de> for ReceiverOutputOpts {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_yaml::Value::deserialize(deserializer)?;
        let value_type = value
            .as_mapping()
            .and_then(|m| m.get("type"))
            .and_then(serde_yaml::Value::as_str)
            .ok_or_else(|| {
                let valid = if cfg!(feature = "kafka") {
                    "http, kafka, rabbitmq, redis, sqs, gcp-pubsub"
                } else {
                    "http, rabbitmq, redis, sqs, gcp-pubsub"
                };
                serde::de::Error::custom(format!(
                    "missing field `type` in receiver output; expected one of: {valid}"
                ))
            })?
            .to_owned();

        match value_type.as_str() {
            "http" => serde_yaml::from_value(value)
                .map(ReceiverOutputOpts::Http)
                .map_err(serde::de::Error::custom),
            #[cfg(feature = "kafka")]
            "kafka" => serde_yaml::from_value(value)
                .map(ReceiverOutputOpts::Kafka)
                .map_err(serde::de::Error::custom),
            "rabbitmq" | "redis" | "sqs" | "gcp-pubsub" => serde_yaml::from_value(value)
                .map(ReceiverOutputOpts::Queue)
                .map_err(serde::de::Error::custom),
            other => {
                let valid = if cfg!(feature = "kafka") {
                    "http, kafka, rabbitmq, redis, sqs, gcp-pubsub"
                } else {
                    "http, rabbitmq, redis, sqs, gcp-pubsub"
                };
                Err(serde::de::Error::custom(format!(
                    "unknown receiver output type `{other}`, expected one of: {valid}"
                )))
            }
        }
    }
}

impl WebhookReceiverConfig {
    pub async fn into_receiver_output(self) -> anyhow::Result<Box<dyn ReceiverOutput>> {
        match self.output {
            ReceiverOutputOpts::Http(opts) => opts.into_receiver_output(self.name),
            #[cfg(feature = "kafka")]
            ReceiverOutputOpts::Kafka(opts) => {
                svix_bridge_plugin_kafka::into_receiver_output(self.name, opts).map_err(Into::into)
            }
            ReceiverOutputOpts::Queue(x) => svix_bridge_plugin_queue::into_receiver_output(
                self.name.clone(),
                x,
                self.transformation.as_ref(),
            )
            .await
            .map_err(Into::into),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum PollerInputOpts {
    SvixMessagePoller {
        /// Identifies this client, allowing the server to track progress during iteration.
        /// Processes should not share a consumer id. Only exclusive access is permitted.
        consumer_id: String,
        /// The app portion of the polling endpoint, e.g. `app_XXXX`
        app_id: String,
        /// The poller portion of the polling endpoint, e.g. `poll_XXXX`
        sink_id: String,
        /// The auth token for the polling endpoint
        token: String,
        #[serde(default)]
        svix_options: Option<SvixOptions>,
    },
}

impl PollerInputOpts {
    pub fn svix_client(&self) -> Option<Svix> {
        match self {
            PollerInputOpts::SvixMessagePoller {
                token,
                svix_options,
                ..
            } => Some(Svix::new(
                token.clone(),
                svix_options.clone().map(Into::into),
            )),
        }
    }
}

/// Config for fetching from HTTP endpoints and forwarding them to plugins.
#[derive(Deserialize)]
pub struct PollerReceiverConfig {
    pub name: String,
    pub input: PollerInputOpts,
    // FIXME: add a configurable polling schedule or interval
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
    pub output: ReceiverOutputOpts,
}

impl PollerReceiverConfig {
    // FIXME: duplicate from WebhookReceiverConfig. Extract/refactor as TryFrom ReceiverOutputOpts?
    pub async fn into_receiver_output(self) -> anyhow::Result<Box<dyn ReceiverOutput>> {
        match self.output {
            ReceiverOutputOpts::Http(opts) => opts.into_receiver_output(self.name),
            #[cfg(feature = "kafka")]
            ReceiverOutputOpts::Kafka(opts) => {
                svix_bridge_plugin_kafka::into_receiver_output(self.name, opts).map_err(Into::into)
            }
            ReceiverOutputOpts::Queue(x) => svix_bridge_plugin_queue::into_receiver_output(
                self.name.clone(),
                x,
                self.transformation.as_ref(),
            )
            .await
            .map_err(Into::into),
        }
    }
}

#[cfg(test)]
mod tests;
