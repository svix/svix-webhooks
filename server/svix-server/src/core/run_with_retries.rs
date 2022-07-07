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
        let fun_ret = fun().await;
        match fun_ret {
            Ok(_) => return fun_ret,
            Err(ref e) => {
                if should_retry(e) {
                    tracing::warn!("Retrying after error {}", e);
                    tokio::time::sleep(*duration).await;
                } else {
                    return fun_ret;
                }
            }
        }
    }

    fun().await
}
