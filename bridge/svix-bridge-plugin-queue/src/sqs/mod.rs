use omniqueue::{DynConsumer, DynProducer, QueueConsumer as _, QueueProducer as _, backends};
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

pub async fn consumer(cfg: &SqsInputOpts) -> DynConsumer {
    backends::SqsBackend::builder(backends::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    })
    .build_consumer()
    .await
    .into_dyn()
}

pub async fn producer(cfg: &SqsOutputOpts) -> DynProducer {
    backends::SqsBackend::builder(backends::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    })
    .build_producer()
    .await
    .into_dyn()
}
