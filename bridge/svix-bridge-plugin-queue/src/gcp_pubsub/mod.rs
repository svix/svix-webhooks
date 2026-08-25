use std::path::PathBuf;

use omniqueue::{DynConsumer, DynProducer, QueueConsumer as _, QueueProducer as _, backends};
use serde::Deserialize;

use crate::error::Result;

#[derive(Debug, Default, Deserialize)]
pub struct GcpPubSubInputOpts {
    pub subscription_id: String,
    pub credentials_file: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GcpPubSubOutputOpts {
    pub topic: String,
    pub credentials_file: Option<PathBuf>,
}

pub async fn consumer(cfg: &GcpPubSubInputOpts) -> Result<DynConsumer> {
    let c = backends::GcpPubSubBackend::builder(backends::GcpPubSubConfig {
        subscription_id: cfg.subscription_id.clone(),
        credentials_file: cfg.credentials_file.clone(),
        // Don't need this. Topics are for producers only.
        topic_id: String::new(),
    })
    .build_consumer()
    .await?
    .into_dyn();

    Ok(c)
}

pub async fn producer(cfg: &GcpPubSubOutputOpts) -> Result<DynProducer> {
    let p = backends::GcpPubSubBackend::builder(backends::GcpPubSubConfig {
        topic_id: cfg.topic.clone(),
        credentials_file: cfg.credentials_file.clone(),
        // Don't need this. Subscriptions are for consumers only.
        subscription_id: String::new(),
    })
    .build_producer()
    .await?
    .into_dyn();

    Ok(p)
}
