use omniqueue::{backends, DynConsumer, DynProducer};
use serde::Deserialize;

use crate::error::{Error, Result};

#[derive(Debug, Default, Deserialize)]
pub struct SqsInputOpts {
    pub queue_dsn: String,
    #[serde(default)]
    pub override_endpoint: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SqsOutputOpts {
    pub queue_dsn: String,
    #[serde(default)]
    pub override_endpoint: bool,
}

pub async fn consumer(cfg: &SqsInputOpts) -> Result<DynConsumer> {
    backends::SqsBackend::builder(backends::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    })
    .make_dynamic()
    .build_consumer()
    .await
    .map_err(Error::from)
}

pub async fn producer(cfg: &SqsOutputOpts) -> Result<DynProducer> {
    backends::SqsBackend::builder(backends::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    })
    .make_dynamic()
    .build_producer()
    .await
    .map_err(Error::from)
}
