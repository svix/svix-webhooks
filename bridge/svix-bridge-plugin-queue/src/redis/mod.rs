use omniqueue::{backends, DynConsumer, DynProducer};
use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Debug, Default, Deserialize)]
pub struct RedisInputOpts {
    pub dsn: String,
    pub max_connections: u16,
    #[serde(default = "default_reinsert_on_nack")]
    pub reinsert_on_nack: bool,
    pub queue_key: String,
    pub delayed_queue_key: Option<String>,
    pub consumer_group: String,
    pub consumer_name: String,
    #[serde(default = "default_ack_deadline_ms")]
    pub ack_deadline_ms: i64,
}

fn default_reinsert_on_nack() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedisOutputOpts {
    pub dsn: String,
    pub max_connections: u16,
    pub queue_key: String,
    pub delayed_queue_key: Option<String>,
    #[serde(default = "default_ack_deadline_ms")]
    pub ack_deadline_ms: i64,
}

fn default_ack_deadline_ms() -> i64 {
    5_000
}

pub async fn consumer(cfg: &RedisInputOpts) -> Result<DynConsumer> {
    let delayed_queue_key = cfg
        .delayed_queue_key
        .clone()
        .unwrap_or_else(|| format!("{}_delays", cfg.queue_key));
    let delayed_lock_key = format!("{delayed_queue_key}_lock");

    backends::RedisBackend::<backends::redis::RedisMultiplexedConnectionManager>::builder(
        backends::RedisConfig {
            dsn: cfg.dsn.clone(),
            max_connections: cfg.max_connections,
            reinsert_on_nack: cfg.reinsert_on_nack,
            queue_key: cfg.queue_key.clone(),
            delayed_queue_key,
            delayed_lock_key,
            consumer_group: cfg.consumer_group.clone(),
            consumer_name: cfg.consumer_name.clone(),
            // FIXME: expose in config?
            payload_key: "payload".to_string(),
            ack_deadline_ms: cfg.ack_deadline_ms,
        },
    )
    .make_dynamic()
    .build_consumer()
    .await
    .map_err(Error::from)
}
pub async fn producer(cfg: &RedisOutputOpts) -> Result<DynProducer> {
    let delayed_queue_key = cfg
        .delayed_queue_key
        .clone()
        .unwrap_or_else(|| format!("{}_delays", cfg.queue_key));
    let delayed_lock_key = format!("{delayed_queue_key}_lock");

    backends::RedisBackend::<backends::redis::RedisMultiplexedConnectionManager>::builder(
        backends::RedisConfig {
            dsn: cfg.dsn.clone(),
            max_connections: cfg.max_connections,
            queue_key: cfg.queue_key.clone(),
            delayed_queue_key,
            delayed_lock_key,
            // FIXME: expose in config?
            payload_key: "payload".to_string(),
            // consumer stuff we don't care about.
            reinsert_on_nack: false,
            consumer_group: String::new(),
            consumer_name: String::new(),
            ack_deadline_ms: cfg.ack_deadline_ms,
        },
    )
    .make_dynamic()
    .build_producer()
    .await
    .map_err(Error::from)
}
