// this file is @generated
#![warn(unreachable_pub)]

mod client;
mod deprecated;

pub use self::client::{
    Svix,
    SvixOptions,
};
pub use crate::models::*;

mod application;
mod authentication;
mod background_task;
mod connector;
mod endpoint;
mod environment;
mod event_type;
mod ingest;
mod ingest_endpoint;
mod ingest_source;
mod integration;
mod message;
mod message_attempt;
mod message_poller;
mod operational_webhook;
mod operational_webhook_endpoint;
mod statistics;
mod streaming;
mod streaming_event_type;
mod streaming_events;
mod streaming_sink;
mod streaming_stream;

#[cfg(feature = "svix_beta")]
pub use self::message::{
    V1MessageEventsParams,
    V1MessageEventsSubscriptionParams,
};
pub use self::{
    application::{
        Application,
        ApplicationCreateOptions,
        ApplicationListOptions,
    },
    authentication::{
        Authentication,
        AuthenticationAppPortalAccessOptions,
        AuthenticationExpireAllOptions,
        AuthenticationLogoutOptions,
        AuthenticationRotateStreamPollerTokenOptions,
        AuthenticationStreamPortalAccessOptions,
    },
    background_task::{
        BackgroundTask,
        BackgroundTaskListOptions,
    },
    connector::{
        Connector,
        ConnectorCreateOptions,
        ConnectorListOptions,
    },
    deprecated::*,
    endpoint::{
        Endpoint,
        EndpointCreateOptions,
        EndpointGetStatsOptions,
        EndpointListOptions,
        EndpointRecoverOptions,
        EndpointReplayMissingOptions,
        EndpointRotateSecretOptions,
        EndpointSendExampleOptions,
    },
    environment::{
        Environment,
        EnvironmentExportOptions,
        EnvironmentImportOptions,
    },
    event_type::{
        EventType,
        EventTypeCreateOptions,
        EventTypeDeleteOptions,
        EventTypeImportOpenapiOptions,
        EventTypeListOptions,
    },
    ingest::{
        Ingest,
        IngestDashboardOptions,
    },
    ingest_endpoint::{
        IngestEndpoint,
        IngestEndpointCreateOptions,
        IngestEndpointListOptions,
        IngestEndpointRotateSecretOptions,
    },
    ingest_source::{
        IngestSource,
        IngestSourceCreateOptions,
        IngestSourceListOptions,
        IngestSourceRotateTokenOptions,
    },
    integration::{
        Integration,
        IntegrationCreateOptions,
        IntegrationListOptions,
        IntegrationRotateKeyOptions,
    },
    message::{
        Message,
        MessageCreateOptions,
        MessageExpungeAllContentsOptions,
        MessageGetOptions,
        MessageListOptions,
    },
    message_attempt::{
        MessageAttempt,
        MessageAttemptListAttemptedDestinationsOptions,
        MessageAttemptListAttemptedMessagesOptions,
        MessageAttemptListByEndpointOptions,
        MessageAttemptListByMsgOptions,
        MessageAttemptResendOptions,
    },
    message_poller::{
        MessagePoller,
        MessagePollerConsumerPollOptions,
        MessagePollerConsumerSeekOptions,
        MessagePollerPollOptions,
    },
    operational_webhook::OperationalWebhook,
    operational_webhook_endpoint::{
        OperationalWebhookEndpoint,
        OperationalWebhookEndpointCreateOptions,
        OperationalWebhookEndpointListOptions,
        OperationalWebhookEndpointRotateSecretOptions,
    },
    statistics::{
        Statistics,
        StatisticsAggregateAppStatsOptions,
    },
    streaming::Streaming,
    streaming_event_type::{
        StreamingEventType,
        StreamingEventTypeCreateOptions,
        StreamingEventTypeDeleteOptions,
        StreamingEventTypeListOptions,
    },
    streaming_events::{
        StreamingEvents,
        StreamingEventsCreateOptions,
        StreamingEventsGetOptions,
    },
    streaming_sink::{
        StreamingSink,
        StreamingSinkCreateOptions,
        StreamingSinkListOptions,
        StreamingSinkRotateSecretOptions,
    },
    streaming_stream::{
        StreamingStream,
        StreamingStreamCreateOptions,
        StreamingStreamListOptions,
    },
};

impl Svix {
    pub fn application(&self) -> Application<'_> {
        Application::new(&self.cfg)
    }

    pub fn authentication(&self) -> Authentication<'_> {
        Authentication::new(&self.cfg)
    }

    pub fn background_task(&self) -> BackgroundTask<'_> {
        BackgroundTask::new(&self.cfg)
    }

    pub fn connector(&self) -> Connector<'_> {
        Connector::new(&self.cfg)
    }

    pub fn endpoint(&self) -> Endpoint<'_> {
        Endpoint::new(&self.cfg)
    }

    pub fn environment(&self) -> Environment<'_> {
        Environment::new(&self.cfg)
    }

    pub fn event_type(&self) -> EventType<'_> {
        EventType::new(&self.cfg)
    }

    pub fn ingest(&self) -> Ingest<'_> {
        Ingest::new(&self.cfg)
    }

    pub fn integration(&self) -> Integration<'_> {
        Integration::new(&self.cfg)
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

    pub fn statistics(&self) -> Statistics<'_> {
        Statistics::new(&self.cfg)
    }

    pub fn streaming(&self) -> Streaming<'_> {
        Streaming::new(&self.cfg)
    }

    #[deprecated = "Use .operational_webhook().endpoint() instead"]
    pub fn operational_webhook_endpoint(&self) -> OperationalWebhookEndpoint<'_> {
        OperationalWebhookEndpoint::new(&self.cfg)
    }
}
