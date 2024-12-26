#[derive(Debug, Clone)]
pub(crate) struct ResponseContent<T> {
    pub status: http02::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

pub(crate) mod application_api;
//pub use self::application_api::{ ApplicationApi, ApplicationApiClient };
pub(crate) mod authentication_api;
//pub use self::authentication_api::{ AuthenticationApi, AuthenticationApiClient };
pub(crate) mod background_tasks_api;
//pub use self::background_tasks_api::{ BackgroundTasksApi, BackgroundTasksApiClient };
pub(crate) mod broadcast_api;
//pub use self::broadcast_api::{ BroadcastApi, BroadcastApiClient };
pub(crate) mod endpoint_api;
//pub use self::endpoint_api::{ EndpointApi, EndpointApiClient };
pub(crate) mod environment_api;
//pub use self::environment_api::{ EnvironmentApi, EnvironmentApiClient };
pub(crate) mod environment_settings_api;
//pub use self::environment_settings_api::{ EnvironmentSettingsApi, EnvironmentSettingsApiClient };
pub(crate) mod event_type_api;
//pub use self::event_type_api::{ EventTypeApi, EventTypeApiClient };
pub(crate) mod events_api;
//pub use self::events_api::{ EventsApi, EventsApiClient };
pub(crate) mod health_api;
//pub use self::health_api::{ HealthApi, HealthApiClient };
pub(crate) mod inbound_api;
//pub use self::inbound_api::{ InboundApi, InboundApiClient };
pub(crate) mod integration_api;
//pub use self::integration_api::{ IntegrationApi, IntegrationApiClient };
pub(crate) mod message_api;
//pub use self::message_api::{ MessageApi, MessageApiClient };
pub(crate) mod message_attempt_api;
//pub use self::message_attempt_api::{ MessageAttemptApi, MessageAttemptApiClient };
pub(crate) mod statistics_api;
//pub use self::statistics_api::{ StatisticsApi, StatisticsApiClient };
pub(crate) mod stream_api;
//pub use self::stream_api::{ StreamApi, StreamApiClient };
pub(crate) mod stream_event_types_api;
//pub use self::stream_event_types_api::{ StreamEventTypesApi, StreamEventTypesApiClient };
pub(crate) mod transformation_template_api;
//pub use self::transformation_template_api::{ TransformationTemplateApi, TransformationTemplateApiClient };
pub(crate) mod webhook_endpoint_api;
//pub use self::webhook_endpoint_api::{ WebhookEndpointApi, WebhookEndpointApiClient };
