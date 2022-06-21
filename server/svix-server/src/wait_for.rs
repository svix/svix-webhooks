use std::time::Duration;

pub async fn wait_for_dsn(
    dsn: &str,
    dependency_name: &str,
    default_port: u16,
    wait_for_seconds: u64,
) {
    let dsn = url::Url::parse(dsn).unwrap_or_else(|_| panic!("Invalid {} DSN", dependency_name));

    let host = dsn
        .host()
        .unwrap_or_else(|| panic!("Expected {} host", dependency_name));
    let port = dsn.port().unwrap_or(default_port);

    let sleep = tokio::time::sleep(Duration::from_secs(wait_for_seconds));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            res = tokio::net::TcpStream::connect((host.to_string(), port)) => {
                if res.is_err() {
                    continue;
                } else {
                    break;
                }
            }

            _ = &mut sleep => {
                panic!("Waiting for host={} port={} timed out", host, port);
            }
        }
    }
}
