mod redis;

pub fn init_metric<T, E: std::fmt::Debug>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(t) => Some(t),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to initialize metric");
            None
        }
    }
}

pub use self::redis::{RedisQueueMetrics, RedisQueueType};
