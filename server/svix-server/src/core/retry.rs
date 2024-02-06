use std::{future::Future, time::Duration};

use tracing::warn;

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
    let mut retry = Retry::new(should_retry, retry_schedule);
    loop {
        if let Some(result) = retry.run(&mut fun).await {
            return result;
        }
    }
}

/// A state machine for retrying an asynchronous operation.
///
/// Unfortunately needed to get around Rust's lack of `AttachedFn*` traits.
/// For usage, check the implementation of `run_with_retries`.`
pub struct Retry<'a, Re> {
    retry_schedule: &'a [Duration],
    should_retry: Re,
}

impl<'a, Re> Retry<'a, Re> {
    pub fn new(should_retry: Re, retry_schedule: &'a [Duration]) -> Self {
        Self {
            retry_schedule,
            should_retry,
        }
    }

    pub async fn run<T, E, F, Fut>(&mut self, f: F) -> Option<Result<T, E>>
    where
        E: std::error::Error,
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        Re: Fn(&E) -> bool,
    {
        match f().await {
            // If the function succeeded, we're done
            Ok(t) => Some(Ok(t)),
            Err(e) => {
                let should_retry = &self.should_retry;
                if self.retry_schedule.is_empty() || !should_retry(&e) {
                    // If we already used up all the retries or should_retry returns false,
                    // return the latest error and stop retrying.
                    self.retry_schedule = &[];
                    Some(Err(e))
                } else {
                    // Otherwise, wait and let the caller call retry.run() again.
                    warn!("Retrying after error: {e}");
                    tokio::time::sleep(self.retry_schedule[0]).await;
                    self.retry_schedule = &self.retry_schedule[1..];
                    None
                }
            }
        }
    }
}
