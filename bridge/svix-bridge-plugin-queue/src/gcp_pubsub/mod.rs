use crate::error::{Error, Result};
use omniqueue::queue::producer::DynProducer;
use omniqueue::{
    backends,
    queue::{consumer::DynConsumer, QueueBackend},
};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize)]
pub struct GCPPubSubInputOpts {
    pub subscription_id: String,
    pub credentials_file: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GCPPubSubOutputOpts {
    pub topic: String,
    pub credentials_file: Option<PathBuf>,
}

pub async fn consumer(cfg: &GCPPubSubInputOpts) -> Result<DynConsumer> {
    backends::gcp_pubsub::GcpPubSubBackend::builder(backends::gcp_pubsub::GcpPubSubConfig {
        subscription_id: cfg.subscription_id.clone(),
        credentials_file: cfg.credentials_file.clone(),
        // Don't need this. Topics are for producers only.
        topic_id: String::new(),
    })
    .make_dynamic()
    .build_consumer()
    .await
    .map_err(Error::from)
}

pub async fn producer(cfg: &GCPPubSubOutputOpts) -> Result<DynProducer> {
    backends::gcp_pubsub::GcpPubSubBackend::builder(backends::gcp_pubsub::GcpPubSubConfig {
        topic_id: cfg.topic.clone(),
        credentials_file: cfg.credentials_file.clone(),
        // Don't need this. Subscriptions are for consumers only.
        subscription_id: String::new(),
    })
    .make_dynamic()
    .build_producer()
    .await
    .map_err(Error::from)
}
