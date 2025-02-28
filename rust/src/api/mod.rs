#![warn(unreachable_pub)]

mod client;

pub use self::client::{Svix, SvixOptions};
pub use crate::models::*;

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
    application::{Application, ApplicationCreateOptions, ApplicationListOptions},
    authentication::{
        Authentication, AuthenticationAppPortalAccessOptions, AuthenticationDashboardAccessOptions,
        AuthenticationExpireAllOptions, AuthenticationLogoutOptions,
    },
    background_task::{BackgroundTask, BackgroundTaskListOptions},
    endpoint::{
        Endpoint, EndpointCreateOptions, EndpointGetStatsOptions, EndpointListOptions,
        EndpointRecoverOptions, EndpointReplayMissingOptions, EndpointRotateSecretOptions,
        EndpointSendExampleOptions,
    },
    event_type::{
        EventType, EventTypeCreateOptions, EventTypeDeleteOptions, EventTypeImportOpenapiOptions,
        EventTypeListOptions,
    },
    integration::{
        Integration, IntegrationCreateOptions, IntegrationListOptions, IntegrationRotateKeyOptions,
    },
    message::{
        Message, MessageCreateOptions, MessageExpungeAllContentsOptions, MessageGetOptions,
        MessageListOptions,
    },
    message_attempt::{
        MessageAttempt, MessageAttemptListAttemptedDestinationsOptions,
        MessageAttemptListAttemptedMessagesOptions, MessageAttemptListByEndpointOptions,
        MessageAttemptListByMsgOptions, MessageAttemptResendOptions,
    },
    operational_webhook_endpoint::{
        OperationalWebhookEndpoint, OperationalWebhookEndpointCreateOptions,
        OperationalWebhookEndpointListOptions,
    },
    statistics::{Statistics, StatisticsAggregateAppStatsOptions},
};

#[deprecated = "Use EndpointGetStatusOptions instead"]
pub type EndpointStatsOptions = EndpointGetStatsOptions;
#[deprecated = "Use MessageAttemptListByMsgOptions instead"]
pub type MessageAttemptListOptions = MessageAttemptListByMsgOptions;
#[deprecated = "Use MessageAttemptListAttemptedDestinationsOptions instead"]
pub type ListOptions = MessageAttemptListAttemptedDestinationsOptions;
#[deprecated = "Use AppUsageStatsIn instead"]
pub type AggregateAppStatsOptions = AppUsageStatsIn;

impl Svix {
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
}
