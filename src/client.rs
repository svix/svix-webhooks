use std::{sync::Arc, time::Duration};

use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};

use crate::{Configuration, connector::make_connector};

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct CoyoteOptions {
    pub debug: bool,

    pub server_url: Option<String>,

    /// Timeout for HTTP requests.
    ///
    /// The timeout is applied from when the request starts connecting until
    /// the response body has finished. If set to `None`, requests never time
    /// out.
    ///
    /// Default: 15 seconds.
    pub timeout: Option<Duration>,

    /// Number of retries
    ///
    /// The number of times the client will retry if a server-side error
    /// or timeout is received.
    ///
    /// Default: 2
    pub num_retries: Option<u32>,

    /// Retry Schedule in milliseconds
    ///
    /// List of delays to wait before each retry attempt.
    /// Takes precedence over `num_retries`.
    pub retry_schedule: Option<Vec<Duration>>,

    /// Proxy address.
    ///
    /// Currently `http://` and `https://` proxies using `HTTP CONNECT`, as well
    /// as `socks5://` and `socks5h://` URLs are supported. The difference
    /// between the last two is that DNS resolution also goes through the proxy
    /// for `socks5h`, but not for `socks5`.
    pub proxy_address: Option<String>,
}

impl Default for CoyoteOptions {
    fn default() -> Self {
        Self {
            debug: false,
            server_url: None,
            timeout: Some(Duration::from_secs(15)),
            num_retries: None,
            retry_schedule: None,
            proxy_address: None,
        }
    }
}

/// Coyote API client.
#[derive(Clone)]
pub struct CoyoteClient {
    pub(super) cfg: Arc<Configuration>,
    server_url: Option<String>,
}

impl CoyoteClient {
    pub fn new(token: String, options: Option<CoyoteOptions>) -> Self {
        let options = options.unwrap_or_default();

        let cfg = Arc::new(Configuration {
            user_agent: Some(format!("coyote-client/{CRATE_VERSION}/rust")),
            client: HyperClient::builder(TokioExecutor::new())
                .build(make_connector(options.proxy_address)),
            timeout: options.timeout,
            // These fields will be set by `with_token` below
            base_path: String::new(),
            bearer_access_token: None,
            num_retries: options.num_retries.unwrap_or(2),
            retry_schedule: options.retry_schedule,
        });
        let client = Self {
            cfg,
            server_url: options.server_url,
        };
        client.with_token(token)
    }

    /// Creates a new `CoyoteClient` API client with a different token,
    /// re-using all of the settings and the Hyper client from
    /// an existing `CoyoteClient` instance.
    ///
    /// This can be used to change the token without incurring
    /// the cost of TLS initialization.
    pub fn with_token(&self, token: String) -> Self {
        let base_path = self
            .server_url
            .clone()
            .unwrap_or_else(|| "http://localhost:8050".to_owned());
        let cfg = Arc::new(Configuration {
            base_path,
            user_agent: self.cfg.user_agent.clone(),
            bearer_access_token: Some(token),
            client: self.cfg.client.clone(),
            timeout: self.cfg.timeout,
            num_retries: self.cfg.num_retries,
            retry_schedule: self.cfg.retry_schedule.clone(),
        });

        Self {
            cfg,
            server_url: self.server_url.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CoyoteClient;
    use crate::models::CacheSetIn;

    #[test]
    fn test_future_send_sync() {
        fn require_send_sync<T: Send + Sync>(_: T) {}

        let client = CoyoteClient::new(String::new(), None);
        let cache_api = client.cache();
        let fut = cache_api.set(CacheSetIn::new(
            "key".to_owned(),
            0,
            "value".as_bytes().to_vec(),
        ));
        require_send_sync(fut);
    }
}
