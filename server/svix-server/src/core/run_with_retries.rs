use std::{future::Future, time::Duration};

pub async fn run_with_retries<T, E, F: Future<Output = Result<T, E>>, FN: FnMut() -> F>(
    mut fun: FN,
    should_retry: impl Fn(&E) -> bool,
    retry_schedule: &[Duration],
) -> Result<T, E> {
    let mut num_attempts = 0;

    loop {
        let fun_ret = fun().await;
        match fun_ret {
            Ok(_) => break fun_ret,
            Err(ref e) => {
                if num_attempts == retry_schedule.len() {
                    break fun_ret;
                } else if should_retry(e) {
                    tokio::time::sleep(retry_schedule[num_attempts]).await;
                    num_attempts += 1;
                } else {
                    break fun_ret;
                }
            }
        }
    }
}
