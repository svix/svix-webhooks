use std::sync::Arc;

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
    pub timeout: Option<std::time::Duration>,
    /// Number of retries
    ///
    /// The number of times the client will retry if a server-side error
    /// or timeout is received.
    ///
    /// Default: 2
    pub num_retries: Option<u32>,
    /// Retry Schedule in milliseconds
    ///
    /// List of delays (in milliseconds) to wait before each retry attempt.
    /// Takes precedence over numRetries.
    pub retry_schedule_in_ms: Option<Vec<u64>>,
}

impl Default for SvixOptions {
    fn default() -> Self {
        Self {
            debug: false,
            server_url: None,
            timeout: Some(std::time::Duration::from_secs(15)),
            num_retries: None,
            retry_schedule_in_ms: None,
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
            client: HyperClient::builder(TokioExecutor::new()).build(crate::default_connector()),
            timeout: options.timeout,
            // These fields will be set by `with_token` below
            base_path: String::new(),
            bearer_access_token: None,
            num_retries: options.num_retries.unwrap_or(2),
            retry_schedule_in_ms: options.retry_schedule_in_ms,
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
            retry_schedule_in_ms: self.cfg.retry_schedule_in_ms.clone(),
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
