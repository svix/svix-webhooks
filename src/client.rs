use std::{sync::Arc, time::Duration};

use hyper::body::Bytes;
use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};

use crate::connector::{Connector, make_connector};

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_URL: &str = "http://localhost:8050";

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

    /// Use HTTP/1.1 on plaintext connections.
    ///
    /// Otherwise forces HTTP/2 on plaintext, and uses standard ALPN on secure connections.
    #[cfg(all(feature = "http1", feature = "http2"))]
    pub http1: bool,
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
            #[cfg(all(feature = "http1", feature = "http2"))]
            http1: false,
        }
    }
}

#[derive(Clone)]
pub(crate) struct Configuration {
    pub(crate) server_url: String,
    pub(crate) user_agent: Option<String>,
    pub(crate) bearer_access_token: Option<String>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) num_retries: u32,
    pub(crate) retry_schedule: Option<Vec<Duration>>,

    pub(crate) client: HyperClient<Connector, http_body_util::Full<Bytes>>,
}

/// Coyote API client.
#[derive(Clone)]
pub struct CoyoteClient {
    pub(super) cfg: Arc<Configuration>,
}

impl CoyoteClient {
    pub fn new(token: String, options: Option<CoyoteOptions>) -> Self {
        let options = options.unwrap_or_default();

        let mut builder = HyperClient::builder(TokioExecutor::new());
        builder.pool_idle_timeout(Duration::from_secs(10));

        #[cfg(all(feature = "http2", not(feature = "http1")))]
        builder.http2_only(true);
        #[cfg(all(feature = "http1", feature = "http2"))]
        builder.http2_only(!options.http1);

        let cfg = Arc::new(Configuration {
            user_agent: Some(format!("coyote-client/{CRATE_VERSION}/rust")),
            client: builder.build(make_connector(options.proxy_address)),
            timeout: options.timeout,
            server_url: match options.server_url {
                None => DEFAULT_URL.to_owned(),
                Some(s) if s.ends_with('/') => s.trim_end_matches('/').to_owned(),
                Some(s) => s,
            },
            bearer_access_token: None,
            num_retries: options.num_retries.unwrap_or(2),
            retry_schedule: options.retry_schedule,
        });
        let client = Self { cfg };
        client.with_token(token)
    }

    /// Creates a new `CoyoteClient` API client with a different token,
    /// re-using all of the settings and the Hyper client from
    /// an existing `CoyoteClient` instance.
    ///
    /// This can be used to change the token without incurring
    /// the cost of TLS initialization.
    pub fn with_token(&self, token: String) -> Self {
        let cfg = Arc::new(Configuration {
            bearer_access_token: Some(token),
            ..(*self.cfg).clone()
        });
        Self { cfg }
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
        let fut = cache_api.set(
            "key".to_owned(),
            CacheSetIn::new("value".as_bytes().to_vec(), 0),
        );
        require_send_sync(fut);
    }
}
