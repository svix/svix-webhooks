import typing as t
from dataclasses import field, dataclass

from .application import Application, ApplicationAsync
from .authentication import Authentication, AuthenticationAsync
from .endpoint import Endpoint, EndpointAsync
from .environment import Environment, EnvironmentAsync
from .event_type import EventType, EventTypeAsync
from .ingest import Ingest, IngestAsync
from .integration import Integration, IntegrationAsync
from .management import Management, ManagementAsync
from .message import Message, MessageAsync
from .message_attempt import MessageAttempt, MessageAttemptAsync
from .operational_webhook import OperationalWebhook, OperationalWebhookAsync
from .operational_webhook_endpoint import (
    OperationalWebhookEndpoint,
    OperationalWebhookEndpointAsync,
)
from .statistics import Statistics, StatisticsAsync
from .client import AuthenticatedClient

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
        elif region == "ca":
            regional_url = "https://api.ca.svix.com"
        elif region == "au":
            regional_url = "https://api.au.svix.com"

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
    def environment(self) -> EnvironmentAsync:
        return EnvironmentAsync(self._client)

    @property
    def event_type(self) -> EventTypeAsync:
        return EventTypeAsync(self._client)

    @property
    def ingest(self) -> IngestAsync:
        return IngestAsync(self._client)

    @property
    def integration(self) -> IntegrationAsync:
        return IntegrationAsync(self._client)

    @property
    def management(self) -> ManagementAsync:
        return ManagementAsync(self._client)

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
    def operational_webhook(self) -> OperationalWebhookAsync:
        return OperationalWebhookAsync(self._client)

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
    def environment(self) -> Environment:
        return Environment(self._client)

    @property
    def event_type(self) -> EventType:
        return EventType(self._client)

    @property
    def ingest(self) -> Ingest:
        return Ingest(self._client)

    @property
    def integration(self) -> Integration:
        return Integration(self._client)

    @property
    def management(self) -> Management:
        return Management(self._client)

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
    def operational_webhook(self) -> OperationalWebhook:
        return OperationalWebhook(self._client)

    @property
    def operational_webhook_endpoint(self) -> OperationalWebhookEndpoint:
        return OperationalWebhookEndpoint(self._client)
