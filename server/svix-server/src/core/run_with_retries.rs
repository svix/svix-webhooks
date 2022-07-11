use std::{future::Future, time::Duration};

pub async fn run_with_retries<
    T,
    E: std::error::Error,
    F: Future<Output = Result<T, E>>,
    FN: FnMut() -> F,
>(
    mut fun: FN,
    should_retry: impl Fn(&E) -> bool,
    retry_schedule: &[Duration],
) -> Result<T, E> {
    for duration in retry_schedule {
        match fun().await {
            Ok(ret) => return Ok(ret),
            Err(e) => {
                if should_retry(&e) {
                    tracing::warn!("Retrying after error {}", e);
                    tokio::time::sleep(*duration).await;
                } else {
                    return Err(e);
                }
            }
        }
    }

    // Loop sleeps after a failed attempt so you need this last fun call to avoid a fencepost error
    // with durations between tries.
    fun().await
}
