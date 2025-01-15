# ruff: noqa: F401

import typing as t
from dataclasses import dataclass, field

from .application import ApplicationAsync, Application, ApplicationListOptions
from .authentication import AuthenticationAsync, Authentication
from .endpoint import EndpointAsync, Endpoint, EndpointListOptions
from .event_type import EventTypeAsync, EventType, EventTypeListOptions
from .integration import IntegrationAsync, Integration
from .message import MessageAsync, Message
from .message_attempt import (
    MessageAttemptAsync,
    MessageAttempt,
    MessageAttemptListOptions,
)
from .operational_webhook import (
    OperationalWebhookEndpointAsync,
    OperationalWebhookEndpoint,
)
from .statistics import StatisticsAsync, Statistics

from svix.internal.openapi_client.models.aggregate_event_types_out import (
    AggregateEventTypesOut,
)
from svix.internal.openapi_client.models.app_usage_stats_in import AppUsageStatsIn
from svix.internal.openapi_client.models.app_usage_stats_out import AppUsageStatsOut
from svix.internal.openapi_client.client import AuthenticatedClient
from svix.internal.openapi_client.models.app_portal_access_in import AppPortalAccessIn
from svix.internal.openapi_client.models.app_portal_access_out import AppPortalAccessOut
from svix.internal.openapi_client.models.application_in import ApplicationIn
from svix.internal.openapi_client.models.application_out import ApplicationOut
from svix.internal.openapi_client.models.application_patch import ApplicationPatch
from svix.internal.openapi_client.models.background_task_out import BackgroundTaskOut
from svix.internal.openapi_client.models.background_task_status import (
    BackgroundTaskStatus,
)
from svix.internal.openapi_client.models.background_task_type import BackgroundTaskType
from svix.internal.openapi_client.models.dashboard_access_out import DashboardAccessOut
from svix.internal.openapi_client.models.endpoint_headers_in import EndpointHeadersIn
from svix.internal.openapi_client.models.endpoint_headers_out import EndpointHeadersOut
from svix.internal.openapi_client.models.endpoint_headers_patch_in import (
    EndpointHeadersPatchIn,
)
from svix.internal.openapi_client.models.endpoint_in import EndpointIn
from svix.internal.openapi_client.models.endpoint_message_out_payload import (
    EndpointMessageOutPayload,
)
from svix.internal.openapi_client.models.endpoint_out import EndpointOut
from svix.internal.openapi_client.models.endpoint_patch import EndpointPatch
from svix.internal.openapi_client.models.endpoint_secret_out import EndpointSecretOut
from svix.internal.openapi_client.models.endpoint_secret_rotate_in import (
    EndpointSecretRotateIn,
)
from svix.internal.openapi_client.models.operational_webhook_endpoint_in import (
    OperationalWebhookEndpointIn,
)
from svix.internal.openapi_client.models.operational_webhook_endpoint_out import (
    OperationalWebhookEndpointOut,
)
from svix.internal.openapi_client.models.operational_webhook_endpoint_secret_in import (
    OperationalWebhookEndpointSecretIn,
)
from svix.internal.openapi_client.models.operational_webhook_endpoint_secret_out import (
    OperationalWebhookEndpointSecretOut,
)
from svix.internal.openapi_client.models.operational_webhook_endpoint_update import (
    OperationalWebhookEndpointUpdate,
)
from svix.internal.openapi_client.models.endpoint_stats import EndpointStats
from svix.internal.openapi_client.models.endpoint_transformation_in import (
    EndpointTransformationIn,
)
from svix.internal.openapi_client.models.endpoint_transformation_out import (
    EndpointTransformationOut,
)
from svix.internal.openapi_client.models.endpoint_update import EndpointUpdate
from svix.internal.openapi_client.models.event_example_in import EventExampleIn
from svix.internal.openapi_client.models.event_type_import_open_api_in import (
    EventTypeImportOpenApiIn,
)
from svix.internal.openapi_client.models.event_type_import_open_api_out import (
    EventTypeImportOpenApiOut,
)
from svix.internal.openapi_client.models.event_type_in import EventTypeIn
from svix.internal.openapi_client.models.event_type_out import EventTypeOut
from svix.internal.openapi_client.models.event_type_patch import EventTypePatch
from svix.internal.openapi_client.models.event_type_update import EventTypeUpdate
from svix.internal.openapi_client.models.integration_in import IntegrationIn
from svix.internal.openapi_client.models.integration_key_out import IntegrationKeyOut
from svix.internal.openapi_client.models.integration_out import IntegrationOut
from svix.internal.openapi_client.models.integration_update import IntegrationUpdate
from svix.internal.openapi_client.models.list_response_application_out import (
    ListResponseApplicationOut,
)
from svix.internal.openapi_client.models.list_response_background_task_out import (
    ListResponseBackgroundTaskOut,
)
from svix.internal.openapi_client.models.list_response_endpoint_message_out import (
    ListResponseEndpointMessageOut,
)
from svix.internal.openapi_client.models.list_response_endpoint_out import (
    ListResponseEndpointOut,
)
from svix.internal.openapi_client.models.list_response_operational_webhook_endpoint_out import (
    ListResponseOperationalWebhookEndpointOut,
)
from svix.internal.openapi_client.models.list_response_event_type_out import (
    ListResponseEventTypeOut,
)
from svix.internal.openapi_client.models.list_response_integration_out import (
    ListResponseIntegrationOut,
)
from svix.internal.openapi_client.models.list_response_message_attempt_endpoint_out import (
    ListResponseMessageAttemptEndpointOut,
)
from svix.internal.openapi_client.models.list_response_message_attempt_out import (
    ListResponseMessageAttemptOut,
)
from svix.internal.openapi_client.models.list_response_message_endpoint_out import (
    ListResponseMessageEndpointOut,
)
from svix.internal.openapi_client.models.list_response_message_out import (
    ListResponseMessageOut,
)
from svix.internal.openapi_client.models.message_attempt_out import MessageAttemptOut
from svix.internal.openapi_client.models.message_in import MessageIn
from svix.internal.openapi_client.models.message_in_payload import MessageInPayload
from svix.internal.openapi_client.models.message_out import MessageOut
from svix.internal.openapi_client.models.message_out_payload import MessageOutPayload
from svix.internal.openapi_client.models.message_status import MessageStatus
from svix.internal.openapi_client.models.ordering import Ordering
from svix.internal.openapi_client.models.recover_in import RecoverIn
from svix.internal.openapi_client.models.recover_out import RecoverOut
from svix.internal.openapi_client.models.replay_in import ReplayIn
from svix.internal.openapi_client.models.replay_out import ReplayOut
from svix.internal.openapi_client.models.status_code_class import StatusCodeClass

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
    "ApplicationIn",
    "ApplicationOut",
    "ApplicationPatch",
    "ListResponseApplicationOut",
    "DashboardAccessOut",
    "EndpointHeadersIn",
    "EndpointHeadersPatchIn",
    "EndpointHeadersOut",
    "EndpointIn",
    "EndpointOut",
    "EndpointPatch",
    "EndpointSecretOut",
    "EndpointSecretRotateIn",
    "ListResponseEndpointOut",
    "EventTypeImportOpenApiIn",
    "EventTypeImportOpenApiOut",
    "EventTypeIn",
    "EventTypeOut",
    "EventTypePatch",
    "EventTypeUpdate",
    "ListResponseEventTypeOut",
    "ListResponseMessageOut",
    "MessageIn",
    "MessageInPayload",
    "MessageOut",
    "MessageOutPayload",
    "EndpointMessageOutPayload",
    "ListResponseMessageAttemptOut",
    "ListResponseEndpointMessageOut",
    "ListResponseMessageEndpointOut",
    "ListResponseMessageAttemptEndpointOut",
    "MessageAttemptOut",
    "MessageStatus",
    "SvixOptions",
    "ApplicationListOptions",
    "EventTypeListOptions",
    "EndpointListOptions",
    "MessageAttemptListOptions",
    "RecoverIn",
    "StatusCodeClass",
    "Svix",
    "SvixAsync",
    "IntegrationKeyOut",
    "IntegrationIn",
    "IntegrationOut",
    "IntegrationUpdate",
    "ListResponseIntegrationOut",
]
