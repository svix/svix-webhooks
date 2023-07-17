use std::time::Duration;

pub async fn wait_for_dsn(
    dsn: &str,
    dependency_name: &str,
    default_port: u16,
    wait_for_seconds: u64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const ATTEMPT_TIMEOUT: Duration = Duration::from_secs(3);
    const RETRY_WAIT: Duration = Duration::from_millis(500);

    let dsn = url::Url::parse(dsn).map_err(|_| format!("Invalid {dependency_name} DSN"))?;

    let host = dsn
        .host()
        .ok_or_else(|| format!("Expected {dependency_name} host"))?;
    let port = dsn.port().unwrap_or(default_port);

    tokio::time::timeout(Duration::from_secs(wait_for_seconds), async {
        loop {
            // Attempt the connection with a timeout
            match tokio::time::timeout(ATTEMPT_TIMEOUT, tokio::net::TcpStream::connect((host.to_string(), port))).await {
                // Connection attempt succeeded
                Ok(Ok(_)) => break,
                // Connection failed before the timeout was reached _or_ timed out
                Ok(Err(_)) | Err(_) => {
                    tracing::debug!("{dependency_name} connection attempt failed, retrying in {RETRY_WAIT:?}...");
                    tokio::time::sleep(RETRY_WAIT).await;
                    continue;
                }
            }
        }
    })
        .await
        .map_err(|_| format!("Waiting for service {dependency_name} host={host} port={port} timed out"))?;

    Ok(())
}
