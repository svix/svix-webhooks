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

fn deserialize_retry_schedule<'de, D>(
    deserializer: D,
) -> std::result::Result<Vec<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    Ok(buf
        .split(',')
        .into_iter()
        .map(|x| Duration::new(x.parse().unwrap(), 0))
        .collect())
}

fn deserialize_redis_dsn<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    let v: Vec<String> = buf.split(',').into_iter().map(|x| x.to_string()).collect();

    if v.is_empty() {
        Ok(None)
    } else {
        Ok(Some(v))
    }
}

const DEFAULTS: &str = r#"
# Default configuration file
# Values here can also be set by setting the appropriate env var, e.g. SVIX_DB_DSN for db_dsn
# The values shown below are the default values. Values commented out are not set but recommended.

# The address to listen on
listen_address = "0.0.0.0:8071"

# The JWT secret for authentication - should be secret and securely generated
# jwt_secret = "8KjzRXrKkd9YFcNyqLSIY8JwiaCeRc6WK4UkMnSW"

# The log level to run the service with. Supported: info, debug, trace
log_level = "info"

# The wanted retry schedule in seconds. Each value is the time to wait between retries.
retry_schedule = "5,300,1800,7200,18000,36000,36000"

# The DSN for the database. Only postgres is currently supported.
# db_dsn = "postgresql://postgres:postgres@pgbouncer/postgres"

# The DSN for redis (can be left empty if not using redis)
# redis_dsn = "redis://redis:6379"

# What kind of message queue to use. Supported: memory, redis (must have redis_dsn configured), rediscluster (add multiple hosts to redis_dsn separated by commas).
queue_type = "redis"

# If true, headers are prefixed with `Webhook-`, otherwise with `Svix-` (default).
whitelabel_headers = false

# How long to wait when making a request (in seconds)
worker_request_timeout = 30

# Should this instance run the API
api_enabled = true

# Should this instance run the message worker
worker_enabled = true
"#;

pub type Configuration = Arc<ConfigurationInner>;

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct ConfigurationInner {
    /// The address to listen on
    pub listen_address: String,
    /// The JWT secret for authentication - should be secret and securely generated
    #[serde(deserialize_with = "deserialize_jwt_secret")]
    pub jwt_secret: Keys,
    /// The log level to run the service with. Supported: info, debug, trace
    pub log_level: LogLevel,
    /// The wanted retry schedule in seconds. Each value is the time to wait between retries.
    #[serde(deserialize_with = "deserialize_retry_schedule")]
    pub retry_schedule: Vec<Duration>,

    /// The DSN for the database. Only postgres is currently supported.
    pub db_dsn: String,

    /// The DSN for redis (can be left empty if not using redis)
    #[serde(deserialize_with = "deserialize_redis_dsn")]
    pub redis_dsn: Option<Vec<String>>,

    /// What kind of message queue to use. Supported: memory, redis (must have redis_dsn configured), rediscluster (add multiple hosts to redis_dsn separated by commas).
    pub queue_type: QueueType,

    /// If true, headers are prefixed with `Webhook-`, otherwise with `Svix-` (default).
    pub whitelabel_headers: bool,

    /// How long to wait when making a request (in seconds)
    #[validate(range(min = 1, max = 30))]
    pub worker_request_timeout: u16,

    // Execution mode
    /// Should this instance run the API
    pub api_enabled: bool,
    /// Should this instance run the message worker
    pub worker_enabled: bool,
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
pub enum QueueType {
    Memory,
    Redis,
    RedisCluster,
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
        .unwrap();

    config.validate().unwrap();
    Ok(Arc::from(config))
}
