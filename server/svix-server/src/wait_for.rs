use std::time::Duration;

pub async fn wait_for_dsn(
    dsn: &str,
    dependency_name: &str,
    default_port: u16,
    wait_for_seconds: u64,
) {
    let dsn = url::Url::parse(dsn).unwrap_or_else(|_| panic!("Invalid {dependency_name} DSN"));

    let host = dsn
        .host()
        .unwrap_or_else(|| panic!("Expected {dependency_name} host"));
    let port = dsn.port().unwrap_or(default_port);

    let sleep = tokio::time::sleep(Duration::from_secs(wait_for_seconds));
    tokio::pin!(sleep);

    const ATTEMPT_TIMEOUT: Duration = Duration::from_secs(3);
    const RETRY_WAIT: Duration = Duration::from_millis(500);

    loop {
        tokio::select! {
            // Attempt the connection with a timeout
            res = tokio::time::timeout(ATTEMPT_TIMEOUT, tokio::net::TcpStream::connect((host.to_string(), port))) => {
                match res {
                    // Connection attempt succeeded
                    Ok(Ok(_)) => break,
                    // Connection failed before the timeout was reached _or_ timed out
                    Ok(Err(_)) | Err(_) => {
                        tracing::debug!("{dependency_name} connection attempt failed, retrying in {RETRY_WAIT:?}...");
                        tokio::time::sleep(RETRY_WAIT).await;
                        continue;
                    },
                }
            }

            _ = &mut sleep => {
                panic!("Waiting for host={host} port={port} timed out");
            }
        }
    }
}
