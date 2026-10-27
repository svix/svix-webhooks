use std::{
    borrow::Cow, collections::HashMap, convert::Infallible, fmt, io::Error, net::SocketAddr,
    num::NonZeroUsize, sync::Arc,
};

use serde::Deserialize;
use shellexpand::LookupError;
#[cfg(feature = "kafka")]
use svix_bridge_plugin_kafka::{KafkaInputOpts, KafkaOutputOpts};
use svix_bridge_plugin_queue::config::{
    GcpPubSubInputOpts, GcpPubSubOutputOpts, RabbitMqInputOpts, RabbitMqOutputOpts, RedisInputOpts,
    RedisOutputOpts, SqsInputOpts, SqsOutputOpts,
};
use svix_bridge_types::{
    ReceiverInputOpts, ReceiverOutput, SenderInput, SenderOutputOpts, SvixOptions,
    TransformationConfig, svix::api::Svix,
};
use tracing::Level;

use crate::{http_output::HttpOutputOpts, runtime::JsExecutor};

#[derive(Clone, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ReceiverInput {
    Webhook {
        path_id: String,
        #[serde(default)]
        verification: svix_bridge_types::WebhookVerifier,
    },
    SvixWebhook {
        path_id: String,
        endpoint_secret: String,
    },
    SvixMessagePoller(PollerInputOpts),
}

#[derive(Clone, Deserialize)]
pub struct Receiver {
    pub name: String,
    pub input: ReceiverInput,
    pub output: ReceiverOutputOpts,
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
}

pub enum EitherReceiver {
    Webhook(WebhookReceiverConfig),
    Poller(PollerReceiverConfig),
}

impl From<Receiver> for EitherReceiver {
    fn from(value: Receiver) -> Self {
        let Receiver {
            name,
            input,
            transformation,
            output,
        } = value;
        match input {
            ReceiverInput::SvixWebhook {
                path_id,
                endpoint_secret,
            } => Self::Webhook(WebhookReceiverConfig {
                name,
                input: ReceiverInputOpts::SvixWebhook {
                    path_id,
                    endpoint_secret,
                },
                transformation,
                output,
            }),
            ReceiverInput::Webhook {
                path_id,
                verification,
            } => Self::Webhook(WebhookReceiverConfig {
                name,
                input: ReceiverInputOpts::Webhook {
                    path_id,
                    verification,
                },
                transformation,
                output,
            }),
            ReceiverInput::SvixMessagePoller(input) => Self::Poller(PollerReceiverConfig {
                name,
                input,
                transformation,
                output,
            }),
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
    pub receivers: Vec<Receiver>,
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
    #[serde(default)]
    pub js_engine: JsExecutor,
}

pub type Configuration = Arc<Config>;

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
        let cfg: Self = serde_saphyr::from_str(&src)
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

        for (name, tc) in cfg.receivers.iter().filter_map(|receiver| {
            receiver
                .transformation
                .as_ref()
                .map(|tc| (&receiver.name, tc))
        }) {
            crate::runtime::validate_script(tc.source().as_str()).map_err(|e| {
                Error::other(format!(
                    "failed to parse transformation for receiver `{name}`: {e:?}"
                ))
            })?;
        }

        Ok(cfg)
    }

    pub fn into_configuration(self) -> Configuration {
        Arc::new(self)
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
#[derive(Deserialize, Clone)]
pub struct WebhookSenderConfig {
    pub name: String,
    pub input: SenderInputOpts,
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
    pub output: SenderOutputOpts,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SenderInputOpts {
    #[cfg(feature = "kafka")]
    Kafka(KafkaInputOpts),
    #[serde(rename = "gcp-pubsub")]
    GcpPubSub(GcpPubSubInputOpts),
    RabbitMQ(RabbitMqInputOpts),
    Redis(RedisInputOpts),
    Sqs(SqsInputOpts),
}

impl WebhookSenderConfig {
    pub fn into_sender_input(self) -> anyhow::Result<Box<dyn SenderInput>> {
        use SenderInputOpts as S;

        let queue = match self.input {
            #[cfg(feature = "kafka")]
            S::Kafka(input_opts) => {
                return Ok(svix_bridge_plugin_kafka::into_sender_input(
                    self.name,
                    input_opts,
                    self.transformation,
                    self.output,
                )?);
            }
            S::GcpPubSub(input_opts) => svix_bridge_plugin_queue::into_sender_input(
                self.name,
                input_opts,
                self.transformation,
                self.output,
            ),
            S::RabbitMQ(input_opts) => svix_bridge_plugin_queue::into_sender_input(
                self.name,
                input_opts,
                self.transformation,
                self.output,
            ),
            S::Redis(input_opts) => svix_bridge_plugin_queue::into_sender_input(
                self.name,
                input_opts,
                self.transformation,
                self.output,
            ),
            S::Sqs(input_opts) => svix_bridge_plugin_queue::into_sender_input(
                self.name,
                input_opts,
                self.transformation,
                self.output,
            ),
        };
        queue.map_err(|e| anyhow::anyhow!("{e}"))
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
#[derive(Clone)]
pub struct WebhookReceiverConfig {
    pub name: String,
    pub input: ReceiverInputOpts,
    pub transformation: Option<TransformationConfig>,
    pub output: ReceiverOutputOpts,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
#[allow(clippy::large_enum_variant)] // we're talking a couple hundred bytes only
pub enum ReceiverOutputOpts {
    Http(HttpOutputOpts),
    #[cfg(feature = "kafka")]
    Kafka(KafkaOutputOpts),
    #[serde(rename = "gcp-pubsub")]
    GcpPubSub(GcpPubSubOutputOpts),
    RabbitMQ(RabbitMqOutputOpts),
    Redis(RedisOutputOpts),
    Sqs(SqsOutputOpts),
}

impl ReceiverOutputOpts {
    async fn into_receiver_output(
        self,
        name: String,
        transformation: Option<&TransformationConfig>,
    ) -> anyhow::Result<Box<dyn ReceiverOutput>> {
        match self {
            ReceiverOutputOpts::Http(opts) => opts.into_receiver_output(name),
            #[cfg(feature = "kafka")]
            ReceiverOutputOpts::Kafka(opts) => {
                svix_bridge_plugin_kafka::into_receiver_output(name, opts).map_err(Into::into)
            }
            ReceiverOutputOpts::GcpPubSub(x) => {
                svix_bridge_plugin_queue::into_receiver_output(name, x, transformation)
                    .await
                    .map_err(Into::into)
            }
            ReceiverOutputOpts::RabbitMQ(x) => {
                svix_bridge_plugin_queue::into_receiver_output(name, x, transformation)
                    .await
                    .map_err(Into::into)
            }
            ReceiverOutputOpts::Redis(x) => {
                svix_bridge_plugin_queue::into_receiver_output(name, x, transformation)
                    .await
                    .map_err(Into::into)
            }
            ReceiverOutputOpts::Sqs(x) => {
                svix_bridge_plugin_queue::into_receiver_output(name, x, transformation)
                    .await
                    .map_err(Into::into)
            }
        }
    }
}

impl WebhookReceiverConfig {
    pub async fn into_receiver_output(self) -> anyhow::Result<Box<dyn ReceiverOutput>> {
        self.output
            .into_receiver_output(self.name, self.transformation.as_ref())
            .await
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct PollerInputOpts {
    /// Identifies this client, allowing the server to track progress during iteration.
    /// Processes should not share a consumer id. Only exclusive access is permitted.
    pub consumer_id: String,
    /// The app portion of the polling endpoint, e.g. `app_XXXX`
    pub app_id: String,
    /// The poller portion of the polling endpoint, e.g. `poll_XXXX`
    pub sink_id: String,
    /// The auth token for the polling endpoint
    pub token: String,
    #[serde(default)]
    pub svix_options: Option<SvixOptions>,
}

impl PollerInputOpts {
    pub fn svix_client(&self) -> Option<Svix> {
        Some(Svix::new(
            self.token.clone(),
            self.svix_options.clone().map(Into::into),
        ))
    }
}

/// Config for fetching from HTTP endpoints and forwarding them to plugins.
#[derive(Clone, Deserialize)]
pub struct PollerReceiverConfig {
    pub name: String,
    pub input: PollerInputOpts,
    // FIXME: add a configurable polling schedule or interval
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
    pub output: ReceiverOutputOpts,
}

impl PollerReceiverConfig {
    pub async fn into_receiver_output(self) -> anyhow::Result<Box<dyn ReceiverOutput>> {
        self.output
            .into_receiver_output(self.name, self.transformation.as_ref())
            .await
    }
}

#[cfg(test)]
mod tests;
