use std::{net::IpAddr, time::Duration};

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver, TokioHandle,
};

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

    let host_ip: IpAddr = match host {
        url::Host::Domain(s) => resolve_host(s).await,
        url::Host::Ipv4(ipv4_addr) => ipv4_addr.into(),
        url::Host::Ipv6(ipv6_addr) => ipv6_addr.into(),
    };

    let sleep = tokio::time::sleep(Duration::from_secs(wait_for_seconds));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            res = tokio::net::TcpStream::connect((host_ip, port)) => {
                if res.is_err() {
                    continue;
                } else {
                    break;
                }
            }

            _ = &mut sleep => {
                panic!("Waiting for host={} ({}) port={} timed out", host, host_ip, port);
            }
        }
    }
}

async fn resolve_host(host: &str) -> IpAddr {
    let resolver = TokioAsyncResolver::new(
        ResolverConfig::default(),
        ResolverOpts::default(),
        TokioHandle,
    )
    .expect("Could not set up DNS resolution");

    let resp = resolver.lookup_ip(host).await.expect("DNS error");
    resp.iter()
        .next()
        .expect("Expected at least one IP address")
}
