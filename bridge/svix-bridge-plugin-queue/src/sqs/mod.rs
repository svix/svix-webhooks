use crate::error::{Error, Result};
use omniqueue::queue::producer::DynProducer;
use omniqueue::{
    backends,
    queue::{consumer::DynConsumer, QueueBackend},
};
use serde::Deserialize;

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
    backends::sqs::SqsQueueBackend::builder(backends::sqs::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    })
    .make_dynamic()
    .build_consumer()
    .await
    .map_err(Error::from)
}

pub async fn producer(cfg: &SqsOutputOpts) -> Result<DynProducer> {
    backends::sqs::SqsQueueBackend::builder(backends::sqs::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    })
    .make_dynamic()
    .build_producer()
    .await
    .map_err(Error::from)
}
