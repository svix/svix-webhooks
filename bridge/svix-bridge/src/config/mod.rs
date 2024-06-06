use std::{
    borrow::Cow,
    collections::HashMap,
    convert::Infallible,
    fmt,
    io::{Error, ErrorKind},
    net::SocketAddr,
    num::NonZeroUsize,
};

use serde::Deserialize;
use shellexpand::LookupError;
use svix_bridge_plugin_queue::config::{
    into_receiver_output, QueueSenderConfig, ReceiverOutputOpts as QueueOutOpts,
};
use svix_bridge_types::{ReceiverInputOpts, ReceiverOutput, SenderInput, TransformationConfig};
use tracing::Level;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Config for reading messages from plugins and forwarding to Svix.
    #[serde(default)]
    pub senders: Vec<WebhookSenderConfig>,
    /// Config for receiving webhooks and forwarding them to plugins.
    #[serde(default)]
    pub receivers: Vec<WebhookReceiverConfig>,
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
                Error::new(
                    ErrorKind::Other,
                    format!("Variable substitution failed: {e}"),
                )
            })?
        } else {
            Cow::Borrowed(raw_src)
        };
        let cfg: Self = serde_yaml::from_str(&src)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to parse config: {}", e)))?;

        for sc in &cfg.senders {
            if let Some(tc) = sc.transformation() {
                crate::runtime::validate_script(tc.source().as_str()).map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!(
                            "failed to parse transformation for sender `{}`: {:?}",
                            &sc.name(),
                            e,
                        ),
                    )
                })?;
            }
        }

        for rc in &cfg.receivers {
            if let Some(tc) = &rc.transformation {
                crate::runtime::validate_script(tc.source().as_str()).map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!(
                            "failed to parse transformation for receiver `{}`: {:?}",
                            &rc.name, e,
                        ),
                    )
                })?;
            }
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
#[serde(untagged)]
pub enum WebhookSenderConfig {
    Queue(QueueSenderConfig),
}

impl WebhookSenderConfig {
    pub fn name(&self) -> &str {
        match self {
            WebhookSenderConfig::Queue(cfg) => &cfg.name,
        }
    }
    pub fn transformation(&self) -> Option<&TransformationConfig> {
        match self {
            WebhookSenderConfig::Queue(cfg) => cfg.transformation.as_ref(),
        }
    }
}

impl TryFrom<WebhookSenderConfig> for Box<dyn SenderInput> {
    type Error = &'static str;
    fn try_from(value: WebhookSenderConfig) -> Result<Self, Self::Error> {
        match value {
            WebhookSenderConfig::Queue(backend) => backend.into_sender_input(),
        }
    }
}

/// Config for receiving webhooks and forwarding them to plugins.
#[derive(Deserialize)]
pub struct WebhookReceiverConfig {
    pub name: String,
    pub input: ReceiverInputOpts,
    #[serde(default)]
    pub transformation: Option<TransformationConfig>,
    pub output: ReceiverOut,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ReceiverOut {
    Queue(QueueOutOpts),
}

impl WebhookReceiverConfig {
    pub async fn into_receiver_output(self) -> std::io::Result<Box<dyn ReceiverOutput>> {
        match self.output {
            ReceiverOut::Queue(x) => {
                into_receiver_output(self.name.clone(), x, self.transformation.as_ref())
                    .await
                    .map_err(Into::into)
            }
        }
    }
}

#[cfg(test)]
mod tests;
