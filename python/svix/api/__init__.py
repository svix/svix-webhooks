import typing as t
from dataclasses import dataclass, field

# add __init__.py from models directory
from ..models import (
    AggregateEventTypesOut,
    ApplicationIn,
    ApplicationOut,
    ApplicationPatch,
    ApplicationTokenExpireIn,
    AppPortalAccessIn,
    AppPortalAccessOut,
    AppUsageStatsIn,
    AppUsageStatsOut,
    BackgroundTaskData,
    BackgroundTaskOut,
    BackgroundTaskStatus,
    BackgroundTaskType,
    DashboardAccessOut,
    EndpointHeadersIn,
    EndpointHeadersOut,
    EndpointHeadersPatchIn,
    EndpointIn,
    EndpointMessageOut,
    EndpointOut,
    EndpointPatch,
    EndpointSecretOut,
    EndpointSecretRotateIn,
    EndpointStats,
    EndpointTransformationIn,
    EndpointTransformationOut,
    EndpointUpdate,
    EnvironmentIn,
    EnvironmentOut,
    EventExampleIn,
    EventTypeFromOpenApi,
    EventTypeImportOpenApiIn,
    EventTypeImportOpenApiOut,
    EventTypeImportOpenApiOutData,
    EventTypeIn,
    EventTypeOut,
    EventTypePatch,
    EventTypeUpdate,
    IntegrationIn,
    IntegrationKeyOut,
    IntegrationOut,
    IntegrationUpdate,
    ListResponseApplicationOut,
    ListResponseBackgroundTaskOut,
    ListResponseEndpointMessageOut,
    ListResponseEndpointOut,
    ListResponseEventTypeOut,
    ListResponseIntegrationOut,
    ListResponseMessageAttemptOut,
    ListResponseMessageEndpointOut,
    ListResponseMessageOut,
    ListResponseOperationalWebhookEndpointOut,
    MessageAttemptOut,
    MessageAttemptTriggerType,
    MessageEndpointOut,
    MessageIn,
    MessageOut,
    MessageStatus,
    OperationalWebhookEndpointHeadersIn,
    OperationalWebhookEndpointHeadersOut,
    OperationalWebhookEndpointIn,
    OperationalWebhookEndpointOut,
    OperationalWebhookEndpointSecretIn,
    OperationalWebhookEndpointSecretOut,
    OperationalWebhookEndpointUpdate,
    Ordering,
    RecoverIn,
    RecoverOut,
    ReplayIn,
    ReplayOut,
    StatusCodeClass,
    TemplateIn,
    TemplateOut,
    TransformationTemplateKind,
)
from .application import (
    Application,
    ApplicationAsync,
    ApplicationCreateOptions,
    ApplicationListOptions,
)
from .authentication import (
    Authentication,
    AuthenticationAppPortalAccessOptions,
    AuthenticationAsync,
    AuthenticationDashboardAccessOptions,
    AuthenticationExpireAllOptions,
    AuthenticationLogoutOptions,
)
from .background_task import (
    BackgroundTask,
    BackgroundTaskAsync,
    BackgroundTaskListOptions,
)
from .client import AuthenticatedClient
from .endpoint import (
    Endpoint,
    EndpointAsync,
    EndpointCreateOptions,
    EndpointGetStatsOptions,
    EndpointListOptions,
    EndpointRecoverOptions,
    EndpointReplayMissingOptions,
    EndpointRotateSecretOptions,
    EndpointSendExampleOptions,
)
from .event_type import (
    EventType,
    EventTypeAsync,
    EventTypeCreateOptions,
    EventTypeDeleteOptions,
    EventTypeImportOpenapiOptions,
    EventTypeListOptions,
)
from .integration import (
    Integration,
    IntegrationAsync,
    IntegrationCreateOptions,
    IntegrationListOptions,
    IntegrationRotateKeyOptions,
)
from .message import (
    Message,
    MessageAsync,
    MessageCreateOptions,
    MessageGetOptions,
    MessageListOptions,
)
from .message_attempt import (
    MessageAttempt,
    MessageAttemptAsync,
    MessageAttemptListAttemptedDestinationsOptions,
    MessageAttemptListAttemptedMessagesOptions,
    MessageAttemptListByEndpointOptions,
    MessageAttemptListByMsgOptions,
    MessageAttemptResendOptions,
)
from .operational_webhook_endpoint import (
    OperationalWebhookEndpoint,
    OperationalWebhookEndpointAsync,
    OperationalWebhookEndpointCreateOptions,
    OperationalWebhookEndpointListOptions,
    OperationalWebhookEndpointRotateSecretOptions,
)
from .statistics import Statistics, StatisticsAggregateAppStatsOptions, StatisticsAsync

DEFAULT_SERVER_URL = "https://api.svix.com"


@dataclass
class SvixOptions:
    debug: bool = False
    server_url: t.Optional[str] = None
    """
    The retry schedule, as seconds to wait after each failed request.

    The first entry is the time in seconds to wait between the first request
    failing and the first retry, and so on.
    Up to five retries are supported, passing a retry schedule with more than
    five entries will raise a `ValueError`.

    Defaults to [0.05, 0.1, 0.2]
    """
    retry_schedule: t.List[float] = field(default_factory=lambda: [0.05, 0.1, 0.2])

    """
    The maximum amount of time in seconds a request can take.

    Request methods will raise httpx.TimeoutException if this is exceeded.
    """
    timeout: float = 15.0


class ClientBase:
    _client: AuthenticatedClient

    def __init__(self, auth_token: str, options: SvixOptions = SvixOptions()) -> None:
        from .. import __version__

        if len(options.retry_schedule) > 5:
            raise ValueError("number of retries must not exceed 5")

        regional_url = None
        region = auth_token.split(".")[-1]
        if region == "us":
            regional_url = "https://api.us.svix.com"
        elif region == "eu":
            regional_url = "https://api.eu.svix.com"
        elif region == "in":
            regional_url = "https://api.in.svix.com"

        host = options.server_url or regional_url or DEFAULT_SERVER_URL
        client = AuthenticatedClient(
            base_url=host,
            token=auth_token,
            headers={"user-agent": f"svix-libs/{__version__}/python"},
            verify_ssl=True,
            retry_schedule=options.retry_schedule,
            timeout=options.timeout,
            follow_redirects=False,
            raise_on_unexpected_status=True,
        )
        self._client = client


class SvixAsync(ClientBase):
    @property
    def authentication(self) -> AuthenticationAsync:
        return AuthenticationAsync(self._client)

    @property
    def application(self) -> ApplicationAsync:
        return ApplicationAsync(self._client)

    @property
    def endpoint(self) -> EndpointAsync:
        return EndpointAsync(self._client)

    @property
    def event_type(self) -> EventTypeAsync:
        return EventTypeAsync(self._client)

    @property
    def integration(self) -> IntegrationAsync:
        return IntegrationAsync(self._client)

    @property
    def message(self) -> MessageAsync:
        return MessageAsync(self._client)

    @property
    def message_attempt(self) -> MessageAttemptAsync:
        return MessageAttemptAsync(self._client)

    @property
    def statistics(self) -> StatisticsAsync:
        return StatisticsAsync(self._client)

    @property
    def operational_webhook_endpoint(self) -> OperationalWebhookEndpointAsync:
        return OperationalWebhookEndpointAsync(self._client)


class Svix(ClientBase):
    @property
    def authentication(self) -> Authentication:
        return Authentication(self._client)

    @property
    def application(self) -> Application:
        return Application(self._client)

    @property
    def endpoint(self) -> Endpoint:
        return Endpoint(self._client)

    @property
    def event_type(self) -> EventType:
        return EventType(self._client)

    @property
    def integration(self) -> Integration:
        return Integration(self._client)

    @property
    def message(self) -> Message:
        return Message(self._client)

    @property
    def message_attempt(self) -> MessageAttempt:
        return MessageAttempt(self._client)

    @property
    def statistics(self) -> Statistics:
        return Statistics(self._client)

    @property
    def operational_webhook_endpoint(self) -> OperationalWebhookEndpoint:
        return OperationalWebhookEndpoint(self._client)


__all__ = [
    "AuthenticatedClient",
    "Application",
    "ApplicationAsync",
    "ApplicationListOptions",
    "ApplicationCreateOptions",
    "Authentication",
    "AuthenticationAsync",
    "AuthenticationAppPortalAccessOptions",
    "AuthenticationExpireAllOptions",
    "AuthenticationDashboardAccessOptions",
    "AuthenticationLogoutOptions",
    "BackgroundTask",
    "BackgroundTaskAsync",
    "BackgroundTaskListOptions",
    "Endpoint",
    "EndpointAsync",
    "EndpointListOptions",
    "EndpointCreateOptions",
    "EndpointRecoverOptions",
    "EndpointReplayMissingOptions",
    "EndpointRotateSecretOptions",
    "EndpointSendExampleOptions",
    "EndpointGetStatsOptions",
    "EventType",
    "EventTypeAsync",
    "EventTypeListOptions",
    "EventTypeCreateOptions",
    "EventTypeImportOpenapiOptions",
    "EventTypeDeleteOptions",
    "Integration",
    "IntegrationAsync",
    "IntegrationListOptions",
    "IntegrationCreateOptions",
    "IntegrationRotateKeyOptions",
    "Message",
    "MessageAsync",
    "MessageListOptions",
    "MessageCreateOptions",
    "MessageGetOptions",
    "MessageAttempt",
    "MessageAttemptAsync",
    "MessageAttemptListByEndpointOptions",
    "MessageAttemptListByMsgOptions",
    "MessageAttemptListAttemptedMessagesOptions",
    "MessageAttemptListAttemptedDestinationsOptions",
    "MessageAttemptResendOptions",
    "OperationalWebhookEndpoint",
    "OperationalWebhookEndpointAsync",
    "OperationalWebhookEndpointListOptions",
    "OperationalWebhookEndpointCreateOptions",
    "OperationalWebhookEndpointRotateSecretOptions",
    "Statistics",
    "StatisticsAsync",
    "StatisticsAggregateAppStatsOptions",
    # These are edited in by hand
    "AggregateEventTypesOut",
    "AppPortalAccessIn",
    "AppPortalAccessOut",
    "AppUsageStatsIn",
    "AppUsageStatsOut",
    "ApplicationIn",
    "ApplicationOut",
    "ApplicationPatch",
    "ApplicationTokenExpireIn",
    "BackgroundTaskData",
    "BackgroundTaskOut",
    "BackgroundTaskStatus",
    "BackgroundTaskType",
    "DashboardAccessOut",
    "EndpointHeadersIn",
    "EndpointHeadersOut",
    "EndpointHeadersPatchIn",
    "EndpointIn",
    "EndpointMessageOut",
    "EndpointOut",
    "EndpointPatch",
    "EndpointSecretOut",
    "EndpointSecretRotateIn",
    "EndpointStats",
    "EndpointTransformationIn",
    "EndpointTransformationOut",
    "EndpointUpdate",
    "EnvironmentIn",
    "EnvironmentOut",
    "EventExampleIn",
    "EventTypeFromOpenApi",
    "EventTypeImportOpenApiIn",
    "EventTypeImportOpenApiOut",
    "EventTypeImportOpenApiOutData",
    "EventTypeIn",
    "EventTypeOut",
    "EventTypePatch",
    "EventTypeUpdate",
    "IntegrationIn",
    "IntegrationKeyOut",
    "IntegrationOut",
    "IntegrationUpdate",
    "ListResponseApplicationOut",
    "ListResponseBackgroundTaskOut",
    "ListResponseEndpointMessageOut",
    "ListResponseEndpointOut",
    "ListResponseEventTypeOut",
    "ListResponseIntegrationOut",
    "ListResponseMessageAttemptOut",
    "ListResponseMessageEndpointOut",
    "ListResponseMessageOut",
    "ListResponseOperationalWebhookEndpointOut",
    "MessageAttemptOut",
    "MessageAttemptTriggerType",
    "MessageEndpointOut",
    "MessageIn",
    "MessageOut",
    "MessageStatus",
    "OperationalWebhookEndpointHeadersIn",
    "OperationalWebhookEndpointHeadersOut",
    "OperationalWebhookEndpointIn",
    "OperationalWebhookEndpointOut",
    "OperationalWebhookEndpointSecretIn",
    "OperationalWebhookEndpointSecretOut",
    "OperationalWebhookEndpointUpdate",
    "Ordering",
    "RecoverIn",
    "RecoverOut",
    "ReplayIn",
    "ReplayOut",
    "StatusCodeClass",
    "TemplateIn",
    "TemplateOut",
    "TransformationTemplateKind",
]
