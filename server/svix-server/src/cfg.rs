// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::sync::Arc;

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use std::time::Duration;

use crate::{core::security::Keys, error::Result};
use serde::{Deserialize, Deserializer};
use tracing::Level;
use validator::Validate;

fn deserialize_jwt_secret<'de, D>(deserializer: D) -> std::result::Result<Keys, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    Ok(Keys::new(buf.as_bytes()))
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RetryScheduleDeserializer {
    Array(Vec<u64>),
    Legacy(String),
}

fn deserialize_retry_schedule<'de, D>(
    deserializer: D,
) -> std::result::Result<Vec<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = RetryScheduleDeserializer::deserialize(deserializer)?;
    match buf {
        RetryScheduleDeserializer::Array(buf) => {
            Ok(buf.into_iter().map(|x| Duration::new(x, 0)).collect())
        }
        RetryScheduleDeserializer::Legacy(buf) => Ok(buf
            .split(',')
            .into_iter()
            .filter_map(|x| {
                let x = x.trim();
                if x.is_empty() {
                    None
                } else {
                    Some(Duration::new(x.parse().expect("Error parsing duration"), 0))
                }
            })
            .collect()),
    }
}

const DEFAULTS: &str = include_str!("../config.default.toml");

pub type Configuration = Arc<ConfigurationInner>;

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct ConfigurationInner {
    /// The address to listen on
    pub listen_address: String,

    /// The address to send operational webhooks to. When None, operational webhooks will not be
    /// sent. When Some, the API server with the given URL will be used to send operational webhooks.
    pub operational_webhook_address: Option<String>,

    /// The JWT secret for authentication - should be secret and securely generated
    #[serde(deserialize_with = "deserialize_jwt_secret")]
    pub jwt_secret: Keys,

    /// The log level to run the service with. Supported: info, debug, trace
    pub log_level: LogLevel,
    /// The log format that all output will follow. Supported: default, json
    pub log_format: LogFormat,
    /// The OpenTelemetry address to send events to if given.
    pub opentelemetry_address: Option<String>,
    /// The ratio at which to sample spans when sending to OpenTelemetry. When not given it defaults
    /// to always sending. If the OpenTelemetry address is not set, this will do nothing.
    pub opentelemetry_sample_ratio: Option<f64>,

    /// The wanted retry schedule in seconds. Each value is the time to wait between retries.
    #[serde(deserialize_with = "deserialize_retry_schedule")]
    pub retry_schedule: Vec<Duration>,

    /// The DSN for the database. Only postgres is currently supported.
    pub db_dsn: String,
    // The maximum number of connections for the PostgreSQL pool
    #[validate(range(min = 10))]
    pub db_pool_max_size: u16,

    /// The DSN for redis (can be left empty if not using redis)
    pub redis_dsn: Option<String>,
    /// The maximum number of connections for the Redis pool
    #[validate(range(min = 10))]
    pub redis_pool_max_size: u16,

    /// What kind of message queue to use. Supported: memory, redis (must have redis_dsn configured).
    pub queue_type: QueueType,

    /// What kind of cache to use. Supported: memory, redis (must have redis_dsn configured), none.
    pub cache_type: CacheType,

    /// If true, headers are prefixed with `Webhook-`, otherwise with `Svix-` (default).
    pub whitelabel_headers: bool,

    /// If true, only allow https endpoints, otherwise also allow http.
    pub endpoint_https_only: bool,

    /// How long to wait when making a request (in seconds)
    #[validate(range(min = 1, max = 30))]
    pub worker_request_timeout: u16,

    // Execution mode
    /// Should this instance run the API
    pub api_enabled: bool,
    /// Should this instance run the message worker
    pub worker_enabled: bool,

    #[serde(flatten)]
    pub internal: InternalConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InternalConfig {
    /// The region to use in the Svix URL given in th dashboard access endpoint
    #[serde(default = "default_region")]
    pub region: String,

    /// The base url to use for the app portal
    #[serde(default = "default_app_portal_url")]
    pub app_portal_url: String,
}

fn default_region() -> String {
    "eu".to_owned()
}

fn default_app_portal_url() -> String {
    "https://app.svix.com".to_owned()
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Debug,
    Trace,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Default,
    Json,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueueType {
    Memory,
    Redis,
    RedisCluster,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheType {
    Memory,
    Redis,
    RedisCluster,
    None,
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
pub fn load() -> Result<Arc<ConfigurationInner>> {
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        // If we have DATABASE_URL set, we should potentially use it.
        const DB_DSN: &str = "SVIX_DB_DSN";
        if std::env::var_os(DB_DSN).is_none() {
            std::env::set_var(DB_DSN, db_url);
        }
    }

    let config: ConfigurationInner = Figment::new()
        .merge(Toml::string(DEFAULTS))
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("SVIX_"))
        .extract()
        .expect("Error loading configuration");

    config.validate().expect("Error validating configuration");
    Ok(Arc::from(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_schedule_parsing() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("SVIX_JWT_SECRET", "x");

            // Multi item
            jail.set_env("SVIX_RETRY_SCHEDULE", "[1,2]");

            let cfg = load().unwrap();
            assert_eq!(
                cfg.retry_schedule,
                vec![Duration::new(1, 0), Duration::new(2, 0)]
            );

            // Single item
            jail.set_env("SVIX_RETRY_SCHEDULE", "[1]");

            let cfg = load().unwrap();
            assert_eq!(cfg.retry_schedule, vec![Duration::new(1, 0)]);

            // Empty
            jail.set_env("SVIX_RETRY_SCHEDULE", "[]");

            let cfg = load().unwrap();
            assert!(cfg.retry_schedule.is_empty());

            Ok(())
        });
    }

    #[test]
    fn test_retry_schedule_parsing_legacy() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("SVIX_JWT_SECRET", "x");

            // Multi item
            jail.set_env("SVIX_RETRY_SCHEDULE", "1,2");

            let cfg = load().unwrap();
            assert_eq!(
                cfg.retry_schedule,
                vec![Duration::new(1, 0), Duration::new(2, 0)]
            );

            // Single item and empty were failing before so not testing them

            Ok(())
        });
    }
}
