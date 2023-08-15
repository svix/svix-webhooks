use crate::error::{Error, Result};
use omniqueue::queue::producer::DynProducer;
use omniqueue::{
    backends,
    queue::{consumer::DynConsumer, QueueBackend},
};

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct RedisInputOpts {
    pub dsn: String,
    pub max_connections: u16,
    #[serde(default = "default_reinsert_on_nack")]
    pub reinsert_on_nack: bool,
    pub queue_key: String,
    pub consumer_group: String,
    pub consumer_name: String,
}

fn default_reinsert_on_nack() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedisOutputOpts {
    pub dsn: String,
    pub max_connections: u16,
    pub queue_key: String,
}

pub async fn consumer(cfg: &RedisInputOpts) -> Result<DynConsumer> {
    backends::redis::RedisQueueBackend::<
        backends::redis::RedisMultiplexedConnectionManager,
    >::builder(backends::redis::RedisConfig {
        dsn: cfg.dsn.clone(),
        max_connections: cfg.max_connections,
        reinsert_on_nack: cfg.reinsert_on_nack,
        queue_key: cfg.queue_key.clone(),
        consumer_group: cfg.consumer_group.clone(),
        consumer_name: cfg.consumer_name.clone(),
        // FIXME: expose in config?
        payload_key: "payload".to_string(),
    })
        .make_dynamic()
        .build_consumer()
        .await
        .map_err(Error::from)
}
pub async fn producer(cfg: &RedisOutputOpts) -> Result<DynProducer> {
    backends::redis::RedisQueueBackend::<
        backends::redis::RedisMultiplexedConnectionManager,
    >::builder(backends::redis::RedisConfig {
        dsn: cfg.dsn.clone(),
        max_connections: cfg.max_connections,
        queue_key: cfg.queue_key.clone(),
        // FIXME: expose in config?
        payload_key: "payload".to_string(),
        // consumer stuff we don't care about.
        reinsert_on_nack: false,
        consumer_group: String::new(),
        consumer_name: String::new(),
    })
        .make_dynamic()
        .build_producer()
        .await.map_err(Error::from)
}
