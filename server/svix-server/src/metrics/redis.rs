use opentelemetry::metrics::{Meter, ObservableGauge};
use redis::{AsyncCommands as _, streams::StreamPendingReply};

use super::init_metric;
use crate::{
    error::{Error, Result},
    redis::RedisManager,
};

pub enum RedisQueueType<'a> {
    Stream(&'a str),
    StreamPending { stream: &'a str, group: &'a str },
    List(&'a str),
    SortedSet(&'a str),
}

impl RedisQueueType<'_> {
    pub async fn queue_depth(&self, redis: &RedisManager) -> Result<u64> {
        let mut conn = redis.get().await?;
        match self {
            RedisQueueType::Stream(q) => conn
                .xlen(q)
                .await
                .map_err(|e| Error::queue(format_args!("Failed to query queue depth: {e}"))),
            RedisQueueType::StreamPending { stream, group } => {
                let reply: StreamPendingReply = conn.xpending(stream, group).await?;
                Ok(reply.count() as _)
            }
            RedisQueueType::List(q) => conn
                .llen(q)
                .await
                .map_err(|e| Error::queue(format_args!("Failed to query queue depth: {e}"))),
            RedisQueueType::SortedSet(q) => conn
                .zcard(q)
                .await
                .map_err(|e| Error::queue(format_args!("Failed to query queue depth: {e}"))),
        }
    }
}

#[derive(Clone)]
pub struct RedisQueueMetrics {
    main_queue: Option<ObservableGauge<u64>>,
    pending_queue: Option<ObservableGauge<u64>>,
    delayed_queue: Option<ObservableGauge<u64>>,
    deadletter_queue: Option<ObservableGauge<u64>>,
}

impl RedisQueueMetrics {
    pub fn new(meter: &Meter) -> Self {
        let main_queue = init_metric(
            meter
                .u64_observable_gauge("svix.queue.depth_main")
                .try_init(),
        );

        let pending_queue = init_metric(
            meter
                .u64_observable_gauge("svix.queue.pending_msgs")
                .try_init(),
        );

        let delayed_queue = init_metric(
            meter
                .u64_observable_gauge("svix.queue.depth_delayed")
                .try_init(),
        );

        let deadletter_queue = init_metric(
            meter
                .u64_observable_gauge("svix.queue.depth_dlq")
                .try_init(),
        );

        Self {
            main_queue,
            pending_queue,
            delayed_queue,
            deadletter_queue,
        }
    }
    pub async fn record(
        &self,
        redis: &RedisManager,
        main_queue: &RedisQueueType<'_>,
        pending_queue: &RedisQueueType<'_>,
        delayed_queue: &RedisQueueType<'_>,
        deadletter_queue: &RedisQueueType<'_>,
    ) {
        main_queue
            .queue_depth(redis)
            .await
            .map(|d| self.record_main_queue_depth(d))
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to record queue depth: {e}");
            });
        pending_queue
            .queue_depth(redis)
            .await
            .map(|d| self.record_pending_queue_depth(d))
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to record queue depth: {e}");
            });
        delayed_queue
            .queue_depth(redis)
            .await
            .map(|d| self.record_delayed_queue_depth(d))
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to record queue depth: {e}");
            });
        deadletter_queue
            .queue_depth(redis)
            .await
            .map(|d| self.record_deadletter_queue_depth(d))
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to record queue depth: {e}");
            });
    }

    fn record_main_queue_depth(&self, value: u64) {
        if let Some(recorder) = &self.main_queue {
            recorder.observe(value, &[]);
        }
    }
    fn record_pending_queue_depth(&self, value: u64) {
        if let Some(recorder) = &self.pending_queue {
            recorder.observe(value, &[]);
        }
    }
    fn record_delayed_queue_depth(&self, value: u64) {
        if let Some(recorder) = &self.delayed_queue {
            recorder.observe(value, &[]);
        }
    }
    fn record_deadletter_queue_depth(&self, value: u64) {
        if let Some(recorder) = &self.deadletter_queue {
            recorder.observe(value, &[]);
        }
    }
}
