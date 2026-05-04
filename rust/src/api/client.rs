use std::{sync::Arc, time::Duration};

use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};

use crate::Configuration;

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct SvixOptions {
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

impl Default for SvixOptions {
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

/// Svix API client.
#[derive(Clone)]
pub struct Svix {
    pub(super) cfg: Arc<Configuration>,
    server_url: Option<String>,
}

impl Svix {
    pub fn new(token: String, options: Option<SvixOptions>) -> Self {
        let options = options.unwrap_or_default();

        let cfg = Arc::new(Configuration {
            user_agent: Some(format!("svix-libs/{CRATE_VERSION}/rust")),
            client: HyperClient::builder(TokioExecutor::new())
                .build(crate::make_connector(options.proxy_address)),
            timeout: options.timeout,
            // These fields will be set by `with_token` below
            base_path: String::new(),
            bearer_access_token: None,
            num_retries: options.num_retries.unwrap_or(2),
            retry_schedule: options.retry_schedule,
        });
        let svix = Self {
            cfg,
            server_url: options.server_url,
        };
        svix.with_token(token)
    }

    /// Creates a new `Svix` API client with a different token,
    /// re-using all of the settings and the Hyper client from
    /// an existing `Svix` instance.
    ///
    /// This can be used to change the token without incurring
    /// the cost of TLS initialization.
    pub fn with_token(&self, token: String) -> Self {
        let base_path = self.server_url.clone().unwrap_or_else(|| {
            match token.split('.').next_back() {
                Some("us") => "https://api.us.svix.com",
                Some("eu") => "https://api.eu.svix.com",
                Some("in") => "https://api.in.svix.com",
                Some("ca") => "https://api.ca.svix.com",
                Some("au") => "https://api.au.svix.com",
                _ => "https://api.svix.com",
            }
            .to_string()
        });
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

    #[cfg(feature = "svix_beta")]
    pub fn cfg(&self) -> &Configuration {
        &self.cfg
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Svix;

    #[test]
    fn test_future_send_sync() {
        fn require_send_sync<T: Send + Sync>(_: T) {}

        let svix = Svix::new(String::new(), None);
        let message_api = svix.message();
        let fut = message_api.expunge_content(String::new(), String::new());
        require_send_sync(fut);
    }
}
