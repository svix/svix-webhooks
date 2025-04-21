// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{borrow::Cow, collections::HashMap, fmt, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{bail, Context};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use ipnet::IpNet;
use serde::{Deserialize, Deserializer};
use tracing::Level;
use url::Url;
use validator::{Validate, ValidationError};

use crate::{
    core::{cryptography::Encryption, security::JwtSigningConfig},
    error::Result,
    v1::utils::validation_error,
};

fn deserialize_main_secret<'de, D>(deserializer: D) -> Result<Encryption, D::Error>
where
    D: Deserializer<'de>,
{
    let key = String::deserialize(deserializer)?;
    // Derive a key so we get a key of the right size
    let key = hmac_sha256::HMAC::mac(b"main", key.as_bytes());
    Ok(Encryption::new(key))
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RetryScheduleDeserializer {
    Array(Vec<u64>),
    Legacy(String),
}

fn deserialize_retry_schedule<'de, D>(deserializer: D) -> Result<Vec<Duration>, D::Error>
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

fn deserialize_hours<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let hours = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(60 * 60 * hours))
}

const DEFAULTS: &str = include_str!("../config.default.toml");

pub type Configuration = Arc<ConfigurationInner>;

fn default_redis_pending_duration_secs() -> u64 {
    45
}

fn validate_operational_webhook_url(url: &str) -> Result<(), ValidationError> {
    match Url::parse(url) {
        Ok(url) => {
            // Verify scheme is http or https
            if url.scheme() != "http" && url.scheme() != "https" {
                return Err(validation_error(
                    Some("operational_webhook_address"),
                    Some("URL scheme must be http or https"),
                ));
            }

            // Verify there's a host
            if url.host().is_none() {
                return Err(validation_error(
                    Some("operational_webhook_address"),
                    Some("URL must include a valid host"),
                ));
            }
        }
        Err(_) => {
            return Err(validation_error(
                Some("operational_webhook_address"),
                Some("Invalid URL format"),
            ));
        }
    }

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Validate)]
#[validate(
    schema(function = "validate_config_complete"),
    skip_on_field_errors = false
)]
pub struct ConfigurationInner {
    /// The address to listen on
    pub listen_address: SocketAddr,

    /// The address to send operational webhooks to. When None, operational webhooks will not be
    /// sent. When Some, the API server with the given URL will be used to send operational webhooks.
    #[validate(custom = "validate_operational_webhook_url")]
    pub operational_webhook_address: Option<String>,

    /// The main secret used by Svix. Used for client-side encryption of sensitive data, etc.
    /// IMPORTANT: Once set, it can't be changed.
    #[serde(
        rename = "main_secret",
        deserialize_with = "deserialize_main_secret",
        default
    )]
    pub encryption: Encryption,

    /// Contains the secret and algorithm for signing JWTs
    #[serde(flatten)]
    pub jwt_signing_config: Arc<JwtSigningConfig>,

    /// This determines the type of key that is generated for endpoint secrets by default (when none is set).
    /// Supported: hmac256 (default), ed25519
    /// Note: this does not affect existing keys, which will continue signing based on the type they were created with.
    pub default_signature_type: DefaultSignatureType,

    /// The log level to run the service with. Supported: info, debug, trace
    pub log_level: LogLevel,
    /// The log format that all output will follow. Supported: default, json
    pub log_format: LogFormat,
    /// The OpenTelemetry address to send events to if given.
    pub opentelemetry_address: Option<String>,
    /// The ratio at which to sample spans when sending to OpenTelemetry. When not given it defaults
    /// to always sending. If the OpenTelemetry address is not set, this will do nothing.
    pub opentelemetry_sample_ratio: Option<f64>,
    /// The service name to use for OpenTelemetry. If not provided, it defaults to "svix_server".
    pub opentelemetry_service_name: String,
    /// Whether to enable the logging of the databases at the configured log level. This may be
    /// useful for analyzing their response times.
    pub db_tracing: bool,
    /// The Sentry DSN to use for error reporting. If this is `None`,
    /// then sentry reporting is disabled
    pub sentry_dsn: Option<sentry::types::Dsn>,
    /// The environment (dev, staging, or prod) that the server is running in.
    pub environment: Environment,

    /// The wanted retry schedule in seconds. Each value is the time to wait between retries.
    #[serde(deserialize_with = "deserialize_retry_schedule")]
    pub retry_schedule: Vec<Duration>,

    /// The DSN for the database. Only postgres is currently supported.
    pub db_dsn: String,
    // The maximum number of connections for the PostgreSQL pool
    #[validate(range(min = 10))]
    pub db_pool_max_size: u16,

    /// The DSN for redis (can be left empty if not using redis)
    /// Note that if using Redis Sentinel, this will be the the DSN
    /// for a Sentinel instance.
    pub redis_dsn: Option<String>,
    /// The maximum number of connections for the Redis pool
    #[validate(range(min = 10))]
    pub redis_pool_max_size: u16,

    #[serde(flatten, default)]
    pub redis_sentinel_cfg: Option<SentinelConfig>,

    /// What kind of message queue to use. Supported: memory, redis (must have redis_dsn or
    /// queue_dsn configured).
    pub queue_type: QueueType,
    /// The DSN for the Redis-backed queue. Overrides `redis_dsn`. (can be left empty if not using
    /// redis)
    pub queue_dsn: Option<String>,

    /// What kind of cache to use. Supported: memory, redis (must have redis_dsn or cache_dsn
    /// configured), none.
    pub cache_type: CacheType,
    /// The DSN for the Redis-backed cache. Overrides `redis_dsn`. (can be left empty if not using
    /// redis)
    pub cache_dsn: Option<String>,

    /// If true, headers are prefixed with `Webhook-`, otherwise with `Svix-` (default).
    pub whitelabel_headers: bool,

    /// If true, only allow https endpoints, otherwise also allow http.
    pub endpoint_https_only: bool,

    /// How long to wait when making a request (in seconds)
    #[validate(range(min = 1, max = 30))]
    pub worker_request_timeout: u16,

    /// How long of a period an endpoint must be consistently failing to be disabled. If a message
    /// is successfully sent during this time, then the endpoint will not disable.
    #[serde(deserialize_with = "deserialize_hours")]
    pub endpoint_failure_disable_after: Duration,

    // Execution mode
    /// Should this instance run the API
    pub api_enabled: bool,
    /// Should this instance run the message worker
    pub worker_enabled: bool,

    /// Subnets to whitelist for outbound webhooks. Note that allowing endpoints in private IP space
    /// is a security risk and should only be allowed if you are using the service internally or for
    /// testing purposes. Should be specified in CIDR notation, e.g., `[127.0.0.1/32, 172.17.0.0/16, 192.168.0.0/16]`
    pub whitelist_subnets: Option<Arc<Vec<IpNet>>>,

    /// Maximum number of concurrent worker tasks to spawn (0 is unlimited)
    pub worker_max_tasks: u16,

    /// Maximum seconds of a queue long-poll
    pub queue_max_poll_secs: u16,

    /// The address of the rabbitmq exchange
    pub rabbit_dsn: Option<Arc<String>>,
    pub rabbit_consumer_prefetch_size: Option<u16>,

    /// Whether or not to completely disable TLS certificate validation on webhook dispatch. This is
    /// a dangerous flag to set true. This value will default to false.
    #[serde(default)]
    pub dangerous_disable_tls_verification: bool,

    /// Optional configuration for sending webhooks through a proxy.
    #[serde(flatten)]
    pub proxy_config: Option<ProxyConfig>,

    #[serde(default = "default_redis_pending_duration_secs")]
    pub redis_pending_duration_secs: u64,

    #[serde(flatten)]
    pub internal: InternalConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ProxyConfig {
    /// Proxy address.
    ///
    /// Currently supported proxy types are:
    /// - `socks5://`, i.e. a SOCKS5 proxy, with domain name resolution being
    ///   done before the proxy gets involved
    /// - `http://` or `https://` proxy, sending HTTP requests to the proxy;
    ///   both HTTP and HTTPS targets are supported
    #[serde(rename = "proxy_addr")]
    pub addr: ProxyAddr,
}

#[derive(Clone, Debug)]
pub enum ProxyAddr {
    /// A SOCKS5 proxy.
    Socks5(http::Uri),
    /// An HTTP / HTTPs proxy.
    Http(http::Uri),
}

impl ProxyAddr {
    pub fn new(raw: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = raw.into();
        let parsed: http::Uri = raw.parse()?;
        match parsed.scheme_str().unwrap_or("") {
            "socks5" => Ok(Self::Socks5(parsed)),
            "http" | "https" => Ok(Self::Http(parsed)),
            _ => Err("Unsupported proxy scheme. \
                Supported schemes are `socks5://`, `http://` and `https://`."
                .into()),
        }
    }
}

impl<'de> Deserialize<'de> for ProxyAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::new(raw).map_err(serde::de::Error::custom)
    }
}

fn validate_config_complete(config: &ConfigurationInner) -> Result<(), ValidationError> {
    match config.cache_type {
        CacheType::None | CacheType::Memory => {}
        CacheType::Redis | CacheType::RedisCluster => {
            if config.cache_dsn().is_none() {
                return Err(ValidationError {
                    code: Cow::from("missing field"),
                    message: Some(Cow::from(
                        "The redis_dsn or cache_dsn field must be set if the cache_type is `redis` or `rediscluster`"
                    )),
                    params: HashMap::new(),
                });
            }
        }
        CacheType::RedisSentinel => {
            if config.cache_dsn().is_none() {
                return Err(ValidationError {
                    code: Cow::from("missing field"),
                    message: Some(Cow::from(
                        "The redis_dsn or cache_dsn field must be set if the cache_type is `redissentinel`"
                    )),
                    params: HashMap::new(),
                });
            }

            if config.redis_sentinel_cfg.is_none() {
                return Err(ValidationError {
                    code: Cow::from("missing field"),
                    message: Some(Cow::from(
                        "sentinel_service_name must be set if the cache_type is `redissentinel`",
                    )),
                    params: HashMap::new(),
                });
            }
        }
    }

    match config.queue_type {
        QueueType::Memory => {}
        QueueType::Redis | QueueType::RedisCluster => {
            if config.queue_dsn().is_none() {
                return Err(ValidationError {
                    code: Cow::from("missing field"),
                    message: Some(Cow::from(
                        "The redis_dsn or queue_dsn field must be set if the queue_type is `redis` or `rediscluster`"
                    )),
                    params: HashMap::new(),
                });
            }
        }
        QueueType::RabbitMQ => {
            if config.rabbit_dsn.is_none() {
                return Err(ValidationError {
                    code: Cow::from("missing field"),
                    message: Some(Cow::from(
                        "The rabbit_dsn field must be set if the queue_type is `rabbitmq`",
                    )),
                    params: HashMap::new(),
                });
            }
        }
        QueueType::RedisSentinel => {
            if config.queue_dsn().is_none() {
                return Err(ValidationError {
                    code: Cow::from("missing field"),
                    message: Some(Cow::from(
                        "The redis_dsn or queue_dsn field must be set if the queue_type is `redissentinel`"
                    )),
                    params: HashMap::new(),
                });
            }

            if config.redis_sentinel_cfg.is_none() {
                return Err(ValidationError {
                    code: Cow::from("missing field"),
                    message: Some(Cow::from(
                        "sentinel_service_name must be set if the queue_type is `redissentinel`",
                    )),
                    params: HashMap::new(),
                });
            }
        }
    }

    Ok(())
}

impl ConfigurationInner {
    pub(self) fn queue_dsn(&self) -> Option<&str> {
        self.queue_dsn.as_deref().or(self.redis_dsn.as_deref())
    }

    pub(self) fn cache_dsn(&self) -> Option<&str> {
        self.cache_dsn.as_deref().or(self.redis_dsn.as_deref())
    }

    /// Fetches the configured backend information for the queue. May panic is the configuration has
    /// not been validated
    pub fn queue_backend(&self) -> QueueBackend<'_> {
        let err = "Called [`queue_backend`] before validating configuration";

        match self.queue_type {
            QueueType::Memory => QueueBackend::Memory,
            QueueType::Redis => QueueBackend::Redis(self.queue_dsn().expect(err)),
            QueueType::RedisCluster => QueueBackend::RedisCluster(self.queue_dsn().expect(err)),
            QueueType::RedisSentinel => QueueBackend::RedisSentinel(
                self.queue_dsn().expect(err),
                self.redis_sentinel_cfg.as_ref().expect(err),
            ),
            QueueType::RabbitMQ => QueueBackend::RabbitMq(self.rabbit_dsn.as_ref().expect(err)),
        }
    }

    /// Fetches the configured backend information for the cache, or `None` if the [`CacheType`] is
    ///  `None`. May panic is the configuration has not been validated
    pub fn cache_backend(&self) -> CacheBackend<'_> {
        let err = "Called [`cache_backend`] before validating configuration";

        match self.cache_type {
            CacheType::None => CacheBackend::None,
            CacheType::Memory => CacheBackend::Memory,
            CacheType::Redis => CacheBackend::Redis(self.cache_dsn().expect(err)),
            CacheType::RedisCluster => CacheBackend::RedisCluster(self.cache_dsn().expect(err)),
            CacheType::RedisSentinel => CacheBackend::RedisSentinel(
                self.cache_dsn().expect(err),
                self.redis_sentinel_cfg.as_ref().expect(err),
            ),
        }
    }
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
    "self_hosted".to_owned()
}

fn default_app_portal_url() -> String {
    "https://app.svix.com".to_owned()
}

#[derive(Debug, Eq, PartialEq)]
pub enum QueueBackend<'a> {
    Memory,
    Redis(&'a str),
    RedisCluster(&'a str),
    RedisSentinel(&'a str, &'a SentinelConfig),
    RabbitMq(&'a str),
}

#[derive(Debug, Eq, PartialEq)]
pub enum CacheBackend<'a> {
    None,
    Memory,
    Redis(&'a str),
    RedisCluster(&'a str),
    RedisSentinel(&'a str, &'a SentinelConfig),
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
    RedisSentinel,
    RabbitMQ,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheType {
    Memory,
    Redis,
    RedisCluster,
    RedisSentinel,
    None,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DefaultSignatureType {
    Hmac256,
    Ed25519,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Environment::Dev => "dev",
                Environment::Staging => "staging",
                Environment::Prod => "prod",
            }
        )
    }
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SentinelConfig {
    #[serde(rename = "sentinel_service_name")]
    pub service_name: String,
    #[serde(default)]
    pub redis_tls_mode_secure: bool,
    pub redis_db: Option<i64>,
    pub redis_username: Option<String>,
    pub redis_password: Option<String>,
    #[serde(default)]
    pub redis_use_resp3: bool,
}

impl From<SentinelConfig> for omniqueue::backends::redis::SentinelConfig {
    fn from(val: SentinelConfig) -> Self {
        let SentinelConfig {
            service_name,
            redis_tls_mode_secure,
            redis_db,
            redis_username,
            redis_password,
            redis_use_resp3,
        } = val;
        omniqueue::backends::redis::SentinelConfig {
            service_name,
            redis_tls_mode_secure,
            redis_db,
            redis_username,
            redis_password,
            redis_use_resp3,
        }
    }
}

/// Try to extract a [`ConfigurationInner`] from the provided [`Figment`]. Any error message should
/// indicate the missing required field(s).
fn try_extract(figment: Figment) -> anyhow::Result<ConfigurationInner> {
    // Explicitly override error if `jwt_secret` is not set, as the default error does not mention
    // the field name due it coming from an inlined field `ConfigurationInner::jwt_signing_config`
    // See: <https://github.com/SergioBenitez/Figment/issues/80>
    if !figment.contains("jwt_secret") {
        bail!("missing field `jwt_secret`");
    }

    Ok(figment.extract()?)
}

pub fn load() -> anyhow::Result<Arc<ConfigurationInner>> {
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        // If we have DATABASE_URL set, we should potentially use it.
        const DB_DSN: &str = "SVIX_DB_DSN";
        if std::env::var_os(DB_DSN).is_none() {
            std::env::set_var(DB_DSN, db_url);
        }
    }

    let merged = Figment::new()
        .merge(Toml::string(DEFAULTS))
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("SVIX_"));

    let config = try_extract(merged).context("failed to extract configuration")?;

    config
        .validate()
        .context("failed to validate configuration")?;
    Ok(Arc::from(config))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use figment::{
        providers::{Format as _, Toml},
        Figment,
    };

    use super::{load, try_extract, CacheBackend, CacheType, QueueBackend, QueueType};
    use crate::core::security::{JWTAlgorithm, JwtSigningConfig};

    #[test]
    fn test_cache_or_queue_dsn_priority() {
        let mut cfg = load().unwrap();
        let cfg = Arc::make_mut(&mut cfg);

        // Override all relevant values
        cfg.queue_type = QueueType::Redis;
        cfg.cache_type = CacheType::Redis;
        cfg.queue_dsn = Some("test_a".to_owned());
        cfg.cache_dsn = Some("test_b".to_owned());
        cfg.redis_dsn = Some("this_value_should_be_overridden".to_owned());

        // Assert that the queue_dsn and cache_dsn overwrite the `redis_dsn`
        assert_eq!(cfg.queue_backend(), QueueBackend::Redis("test_a"));
        assert_eq!(cfg.cache_backend(), CacheBackend::Redis("test_b"));
    }

    #[test]
    fn test_try_extract_missing_jwt_secret() {
        let defaults = Figment::new();

        let actual = try_extract(defaults);

        let err = actual.unwrap_err();
        assert_eq!(err.to_string(), "missing field `jwt_secret`");
    }

    #[test]
    fn test_jwt_signing_fallback() {
        let raw_config = r#"
jwt_secret = "not_actually_a_secret"
        "#;

        let actual: JwtSigningConfig = Figment::new()
            .merge(Toml::string(raw_config))
            .extract()
            .unwrap();

        assert!(matches!(actual, JwtSigningConfig::Default { .. }));
    }

    #[test]
    fn test_jwt_select_algorithm() {
        let raw_config = r#"
jwt_secret = "not_actually_a_secret"
jwt_algorithm = "HS512"
        "#;

        let actual: JwtSigningConfig = Figment::new()
            .merge(Toml::string(raw_config))
            .extract()
            .unwrap();

        assert!(matches!(
            actual,
            JwtSigningConfig::Advanced(JWTAlgorithm::HS512(_))
        ));
    }
}
