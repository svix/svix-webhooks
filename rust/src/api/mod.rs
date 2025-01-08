#![warn(unreachable_pub)]

use std::sync::Arc;

use hyper_util::{client::legacy::Client as HyperClient, rt::TokioExecutor};

use crate::Configuration;

pub use crate::models::*;

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

mod application;
mod authentication;
mod background_task;
mod endpoint;
mod event_type;
mod integration;
mod message;
mod message_attempt;
mod operational_webhook_endpoint;
mod statistics;

#[cfg(feature = "svix_beta")]
pub use self::message::{V1MessageEventsParams, V1MessageEventsSubscriptionParams};
pub use self::{
    application::{Application, ApplicationListOptions},
    authentication::Authentication,
    background_task::{BackgroundTask, BackgroundTaskListOptions},
    endpoint::{Endpoint, EndpointGetStatsOptions, EndpointListOptions},
    event_type::{EventType, EventTypeListOptions},
    integration::{Integration, IntegrationListOptions},
    message::{Message, MessageListOptions},
    message_attempt::{
        MessageAttempt, MessageAttemptListAttemptedDestinationsOptions,
        MessageAttemptListAttemptedMessagesOptions, MessageAttemptListByEndpointOptions,
        MessageAttemptListByMsgOptions,
    },
    operational_webhook_endpoint::{
        OperationalWebhookEndpoint, OperationalWebhookEndpointListOptions,
    },
    statistics::Statistics,
};

#[deprecated = "Use EndpointGetStatusOptions instead"]
pub type EndpointStatsOptions = EndpointGetStatsOptions;
#[deprecated = "Use MessageAttemptListByMsgOptions instead"]
pub type MessageAttemptListOptions = MessageAttemptListByMsgOptions;
#[deprecated = "Use MessageAttemptListAttemptedDestinationsOptions instead"]
pub type ListOptions = MessageAttemptListAttemptedDestinationsOptions;
#[deprecated = "Use AppUsageStatsIn instead"]
pub type AggregateAppStatsOptions = AppUsageStatsIn;

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
}

impl Default for SvixOptions {
    fn default() -> Self {
        Self {
            debug: false,
            server_url: None,
            timeout: Some(std::time::Duration::from_secs(15)),
        }
    }
}

/// Svix API client.
#[derive(Clone)]
pub struct Svix {
    cfg: Arc<Configuration>,
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
            match token.split('.').last() {
                Some("us") => "https://api.us.svix.com",
                Some("eu") => "https://api.eu.svix.com",
                Some("in") => "https://api.in.svix.com",
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
        });

        Self {
            cfg,
            server_url: self.server_url.clone(),
        }
    }

    pub fn authentication(&self) -> Authentication<'_> {
        Authentication::new(&self.cfg)
    }

    pub fn application(&self) -> Application<'_> {
        Application::new(&self.cfg)
    }

    pub fn background_task(&self) -> BackgroundTask<'_> {
        BackgroundTask::new(&self.cfg)
    }

    pub fn endpoint(&self) -> Endpoint<'_> {
        Endpoint::new(&self.cfg)
    }

    pub fn integration(&self) -> Integration<'_> {
        Integration::new(&self.cfg)
    }

    pub fn event_type(&self) -> EventType<'_> {
        EventType::new(&self.cfg)
    }

    pub fn message(&self) -> Message<'_> {
        Message::new(&self.cfg)
    }

    pub fn message_attempt(&self) -> MessageAttempt<'_> {
        MessageAttempt::new(&self.cfg)
    }

    pub fn operational_webhook_endpoint(&self) -> OperationalWebhookEndpoint<'_> {
        OperationalWebhookEndpoint::new(&self.cfg)
    }

    pub fn statistics(&self) -> Statistics<'_> {
        Statistics::new(&self.cfg)
    }

    #[cfg(feature = "svix_beta")]
    pub fn cfg(&self) -> &Configuration {
        &self.cfg
    }
}

#[derive(Default)]
pub struct PostOptions {
    pub idempotency_key: Option<String>,
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
