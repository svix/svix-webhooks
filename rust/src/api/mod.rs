#![warn(unreachable_pub)]

mod client;

pub use self::client::{Svix, SvixOptions};
pub use crate::models::*;

mod application;
mod authentication;
mod background_task;
mod deprecated;
mod endpoint;
mod environment;
mod event_type;
#[cfg(feature = "svix_beta")]
mod events_public;
mod ingest;
mod ingest_endpoint;
mod ingest_source;
mod integration;
mod management;
mod management_authentication;
mod message;
mod message_attempt;
mod message_poller;
mod operational_webhook;
mod operational_webhook_endpoint;
mod statistics;

pub use self::{
    application::{Application, ApplicationCreateOptions, ApplicationListOptions},
    authentication::{
        Authentication, AuthenticationAppPortalAccessOptions, AuthenticationExpireAllOptions,
        AuthenticationLogoutOptions,
    },
    background_task::{BackgroundTask, BackgroundTaskListOptions},
    deprecated::*,
    endpoint::{
        Endpoint, EndpointCreateOptions, EndpointGetStatsOptions, EndpointListOptions,
        EndpointRecoverOptions, EndpointReplayMissingOptions, EndpointRotateSecretOptions,
        EndpointSendExampleOptions,
    },
    environment::{Environment, EnvironmentExportOptions, EnvironmentImportOptions},
    event_type::{
        EventType, EventTypeCreateOptions, EventTypeDeleteOptions, EventTypeImportOpenapiOptions,
        EventTypeListOptions,
    },
    ingest::{Ingest, IngestDashboardOptions},
    ingest_endpoint::{
        IngestEndpoint, IngestEndpointCreateOptions, IngestEndpointListOptions,
        IngestEndpointRotateSecretOptions,
    },
    ingest_source::{
        IngestSource, IngestSourceCreateOptions, IngestSourceListOptions,
        IngestSourceRotateTokenOptions,
    },
    integration::{
        Integration, IntegrationCreateOptions, IntegrationListOptions, IntegrationRotateKeyOptions,
    },
    management::Management,
    management_authentication::{
        ManagementAuthentication, ManagementAuthenticationCreateApiTokenOptions,
        ManagementAuthenticationExpireApiTokenOptions,
        ManagementAuthenticationListApiTokensOptions,
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
    message_poller::{
        MessagePoller, MessagePollerConsumerPollOptions, MessagePollerConsumerSeekOptions,
        MessagePollerPollOptions,
    },
    operational_webhook::OperationalWebhook,
    operational_webhook_endpoint::{
        OperationalWebhookEndpoint, OperationalWebhookEndpointCreateOptions,
        OperationalWebhookEndpointListOptions, OperationalWebhookEndpointRotateSecretOptions,
    },
    statistics::{Statistics, StatisticsAggregateAppStatsOptions},
};
#[cfg(feature = "svix_beta")]
pub use self::{
    events_public::{EventsPublic, EventsPublicConsumerOptions, EventsPublicConsumerSeekOptions},
    message::{V1MessageEventsParams, V1MessageEventsSubscriptionParams},
};

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

    pub fn environment(&self) -> Environment<'_> {
        Environment::new(&self.cfg)
    }

    #[cfg(feature = "svix_beta")]
    pub fn events_public(&self) -> EventsPublic<'_> {
        EventsPublic::new(&self.cfg)
    }

    pub fn ingest(&self) -> Ingest<'_> {
        Ingest::new(&self.cfg)
    }

    pub fn integration(&self) -> Integration<'_> {
        Integration::new(&self.cfg)
    }

    pub fn event_type(&self) -> EventType<'_> {
        EventType::new(&self.cfg)
    }

    pub fn management(&self) -> Management<'_> {
        Management::new(&self.cfg)
    }

    pub fn message(&self) -> Message<'_> {
        Message::new(&self.cfg)
    }

    pub fn message_attempt(&self) -> MessageAttempt<'_> {
        MessageAttempt::new(&self.cfg)
    }

    pub fn operational_webhook(&self) -> OperationalWebhook<'_> {
        OperationalWebhook::new(&self.cfg)
    }

    pub fn operational_webhook_endpoint(&self) -> OperationalWebhookEndpoint<'_> {
        OperationalWebhookEndpoint::new(&self.cfg)
    }

    pub fn statistics(&self) -> Statistics<'_> {
        Statistics::new(&self.cfg)
    }
}
