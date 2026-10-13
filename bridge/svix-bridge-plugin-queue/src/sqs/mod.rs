use omniqueue::{DynConsumer, DynProducer, QueueConsumer as _, QueueProducer as _, backends};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct SqsInputOpts {
    pub queue_dsn: String,
    #[serde(default)]
    pub override_endpoint: bool,
    /// Maximum message size in bytes. Send attempts over this size fail locally without ever
    /// reaching SQS. Defaults to the maximum payload size SQS itself allows (1,048,576 bytes).
    #[serde(default)]
    pub max_payload_size: Option<usize>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct SqsOutputOpts {
    pub queue_dsn: String,
    #[serde(default)]
    pub override_endpoint: bool,
    /// Maximum message size in bytes. Send attempts over this size fail locally without ever
    /// reaching SQS. Defaults to the maximum payload size SQS itself allows (1,048,576 bytes).
    #[serde(default)]
    pub max_payload_size: Option<usize>,
}

pub async fn consumer(cfg: &SqsInputOpts) -> DynConsumer {
    let mut builder = backends::SqsBackend::builder(backends::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    });
    if let Some(max_payload_size) = cfg.max_payload_size {
        builder = builder.max_payload_size(max_payload_size);
    }
    builder.build_consumer().await.into_dyn()
}

pub async fn producer(cfg: &SqsOutputOpts) -> DynProducer {
    let mut builder = backends::SqsBackend::builder(backends::SqsConfig {
        queue_dsn: cfg.queue_dsn.clone(),
        override_endpoint: cfg.override_endpoint,
    });
    if let Some(max_payload_size) = cfg.max_payload_size {
        builder = builder.max_payload_size(max_payload_size);
    }
    builder.build_producer().await.into_dyn()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Proves `max_payload_size` is actually wired into the builder, not just parsed from
    /// config: the size check happens locally (no network call), so this needs AWS env vars
    /// present for config resolution (see `run-tests.sh`) but no live SQS/ElasticMQ connection.
    #[tokio::test]
    async fn producer_respects_max_payload_size() {
        let cfg = SqsOutputOpts {
            queue_dsn: "http://localhost:9324/000000000000/local".to_string(),
            override_endpoint: true,
            max_payload_size: Some(10),
        };
        let p = producer(&cfg).await;

        let err = p
            .send_serde_json(&serde_json::json!({"hello": "this payload is well over ten bytes"}))
            .await
            .expect_err("payload over max_payload_size should be rejected locally");

        assert!(
            matches!(
                err,
                omniqueue::QueueError::PayloadTooLarge { limit: 10, .. }
            ),
            "expected PayloadTooLarge{{ limit: 10, .. }}, got {err:?}"
        );
    }
}
