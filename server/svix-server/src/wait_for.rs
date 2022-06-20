use std::{net::IpAddr, time::Duration};

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver, TokioHandle,
};

use crate::cfg::Configuration;

const POSTGRES_NAME: &str = "PostgreSQL";
const POSTGRES_PORT: u16 = 5432;

const REDIS_NAME: &str = "Redis";
const REDIS_PORT: u16 = 6379;

pub async fn wait_for(cfg: &Configuration, wait_for_seconds: u64) {
    if let Some(redis_dsn) = &cfg.redis_dsn {
        tokio::join!(
            wait_for_inner(&cfg.db_dsn, POSTGRES_NAME, POSTGRES_PORT, wait_for_seconds),
            wait_for_inner(redis_dsn, REDIS_NAME, REDIS_PORT, wait_for_seconds),
        );
    } else {
        wait_for_inner(&cfg.db_dsn, POSTGRES_NAME, POSTGRES_PORT, wait_for_seconds).await
    }
}

async fn wait_for_inner(
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

    let host: IpAddr = match host {
        url::Host::Domain(s) => resolve_host(s).await,
        url::Host::Ipv4(ipv4_addr) => ipv4_addr.into(),
        url::Host::Ipv6(ipv6_addr) => ipv6_addr.into(),
    };

    let sleep = tokio::time::sleep(Duration::from_secs(wait_for_seconds));
    tokio::pin!(sleep);

    loop {
        tokio::select! {
            res = tokio::net::TcpStream::connect((host, port)) => {
                if res.is_err() {
                    continue;
                } else {
                    break;
                }
            }

            _ = &mut sleep => {
                panic!("Waiting for {} timed out", dependency_name);
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
