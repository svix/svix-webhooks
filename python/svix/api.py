import typing as t
from dataclasses import asdict, dataclass, field
from datetime import datetime, timezone

from deprecated import deprecated

from svix.internal.openapi_client.models.aggregate_event_types_out import (
    AggregateEventTypesOut,
)
from svix.internal.openapi_client.models.app_usage_stats_in import AppUsageStatsIn
from svix.internal.openapi_client.models.app_usage_stats_out import AppUsageStatsOut

from .internal.openapi_client.api.application import (
    v1_application_create,
    v1_application_delete,
    v1_application_get,
    v1_application_list,
    v1_application_patch,
    v1_application_update,
)
from .internal.openapi_client.api.authentication import (
    v1_authentication_app_portal_access,
    v1_authentication_dashboard_access,
    v1_authentication_logout,
)
from .internal.openapi_client.api.background_tasks import (
    get_background_task,
    list_background_tasks,
)
from .internal.openapi_client.api.endpoint import (
    v1_endpoint_create,
    v1_endpoint_delete,
    v1_endpoint_get,
    v1_endpoint_get_headers,
    v1_endpoint_get_secret,
    v1_endpoint_get_stats,
    v1_endpoint_list,
    v1_endpoint_patch,
    v1_endpoint_patch_headers,
    v1_endpoint_recover,
    v1_endpoint_replay,
    v1_endpoint_rotate_secret,
    v1_endpoint_send_example,
    v1_endpoint_transformation_get,
    v1_endpoint_transformation_partial_update,
    v1_endpoint_update,
    v1_endpoint_update_headers,
)
from .internal.openapi_client.api.event_type import (
    v1_event_type_create,
    v1_event_type_delete,
    v1_event_type_get,
    v1_event_type_import_openapi,
    v1_event_type_list,
    v1_event_type_patch,
    v1_event_type_update,
)
from .internal.openapi_client.api.integration import (
    v1_integration_create,
    v1_integration_delete,
    v1_integration_get,
    v1_integration_get_key,
    v1_integration_list,
    v1_integration_rotate_key,
    v1_integration_update,
)
from .internal.openapi_client.api.message import (
    v1_message_create,
    v1_message_expunge_content,
    v1_message_get,
    v1_message_list,
)
from .internal.openapi_client.api.message_attempt import (
    v1_message_attempt_expunge_content,
    v1_message_attempt_get,
    v1_message_attempt_list_attempted_destinations,
    v1_message_attempt_list_attempted_messages,
    v1_message_attempt_list_by_endpoint,
    v1_message_attempt_list_by_endpoint_deprecated,
    v1_message_attempt_list_by_msg,
    v1_message_attempt_resend,
)
from .internal.openapi_client.api.statistics import (
    v1_statistics_aggregate_app_stats,
    v1_statistics_aggregate_event_types,
)
from .internal.openapi_client.client import AuthenticatedClient
from .internal.openapi_client.models.app_portal_access_in import AppPortalAccessIn
from .internal.openapi_client.models.app_portal_access_out import AppPortalAccessOut
from .internal.openapi_client.models.application_in import ApplicationIn
from .internal.openapi_client.models.application_out import ApplicationOut
from .internal.openapi_client.models.application_patch import ApplicationPatch
from .internal.openapi_client.models.background_task_out import BackgroundTaskOut
from .internal.openapi_client.models.background_task_status import BackgroundTaskStatus
from .internal.openapi_client.models.background_task_type import BackgroundTaskType
from .internal.openapi_client.models.dashboard_access_out import DashboardAccessOut
from .internal.openapi_client.models.endpoint_headers_in import EndpointHeadersIn
from .internal.openapi_client.models.endpoint_headers_out import EndpointHeadersOut
from .internal.openapi_client.models.endpoint_headers_patch_in import (
    EndpointHeadersPatchIn,
)
from .internal.openapi_client.models.endpoint_in import EndpointIn
from .internal.openapi_client.models.endpoint_message_out_payload import (
    EndpointMessageOutPayload,
)
from .internal.openapi_client.models.endpoint_out import EndpointOut
from .internal.openapi_client.models.endpoint_patch import EndpointPatch
from .internal.openapi_client.models.endpoint_secret_out import EndpointSecretOut
from .internal.openapi_client.models.endpoint_secret_rotate_in import (
    EndpointSecretRotateIn,
)
from .internal.openapi_client.models.endpoint_stats import EndpointStats
from .internal.openapi_client.models.endpoint_transformation_in import (
    EndpointTransformationIn,
)
from .internal.openapi_client.models.endpoint_transformation_out import (
    EndpointTransformationOut,
)
from .internal.openapi_client.models.endpoint_update import EndpointUpdate
from .internal.openapi_client.models.event_example_in import EventExampleIn
from .internal.openapi_client.models.event_type_import_open_api_in import (
    EventTypeImportOpenApiIn,
)
from .internal.openapi_client.models.event_type_import_open_api_out import (
    EventTypeImportOpenApiOut,
)
from .internal.openapi_client.models.event_type_in import EventTypeIn
from .internal.openapi_client.models.event_type_out import EventTypeOut
from .internal.openapi_client.models.event_type_patch import EventTypePatch
from .internal.openapi_client.models.event_type_update import EventTypeUpdate
from .internal.openapi_client.models.integration_in import IntegrationIn
from .internal.openapi_client.models.integration_key_out import IntegrationKeyOut
from .internal.openapi_client.models.integration_out import IntegrationOut
from .internal.openapi_client.models.integration_update import IntegrationUpdate
from .internal.openapi_client.models.list_response_application_out import (
    ListResponseApplicationOut,
)
from .internal.openapi_client.models.list_response_background_task_out import (
    ListResponseBackgroundTaskOut,
)
from .internal.openapi_client.models.list_response_endpoint_message_out import (
    ListResponseEndpointMessageOut,
)
from .internal.openapi_client.models.list_response_endpoint_out import (
    ListResponseEndpointOut,
)
from .internal.openapi_client.models.list_response_event_type_out import (
    ListResponseEventTypeOut,
)
from .internal.openapi_client.models.list_response_integration_out import (
    ListResponseIntegrationOut,
)
from .internal.openapi_client.models.list_response_message_attempt_endpoint_out import (
    ListResponseMessageAttemptEndpointOut,
)
from .internal.openapi_client.models.list_response_message_attempt_out import (
    ListResponseMessageAttemptOut,
)
from .internal.openapi_client.models.list_response_message_endpoint_out import (
    ListResponseMessageEndpointOut,
)
from .internal.openapi_client.models.list_response_message_out import (
    ListResponseMessageOut,
)
from .internal.openapi_client.models.message_attempt_out import MessageAttemptOut
from .internal.openapi_client.models.message_in import MessageIn
from .internal.openapi_client.models.message_in_payload import MessageInPayload
from .internal.openapi_client.models.message_out import MessageOut
from .internal.openapi_client.models.message_out_payload import MessageOutPayload
from .internal.openapi_client.models.message_status import MessageStatus
from .internal.openapi_client.models.ordering import Ordering
from .internal.openapi_client.models.recover_in import RecoverIn
from .internal.openapi_client.models.recover_out import RecoverOut
from .internal.openapi_client.models.replay_in import ReplayIn
from .internal.openapi_client.models.replay_out import ReplayOut
from .internal.openapi_client.models.status_code_class import StatusCodeClass

DEFAULT_SERVER_URL = "https://api.svix.com"


def ensure_tz(x: t.Optional[datetime]) -> t.Optional[datetime]:
    if x is None:
        return None

    if x.tzinfo is None:
        return x.replace(tzinfo=timezone.utc)
    return x


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


@dataclass
class ListOptions:
    iterator: t.Optional[str] = None
    limit: t.Optional[int] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: v for k, v in asdict(self).items() if v is not None}


@dataclass
class PostOptions:
    idempotency_key: t.Optional[str] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: v for k, v in asdict(self).items() if v is not None}


@dataclass
class MessageListOptions(ListOptions):
    event_types: t.Optional[t.List[str]] = None
    before: t.Optional[datetime] = None
    after: t.Optional[datetime] = None
    channel: t.Optional[str] = None
    tag: t.Optional[str] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        d = super().to_dict()
        if self.before is not None:
            d["before"] = ensure_tz(self.before)
        if self.after is not None:
            d["after"] = ensure_tz(self.after)
        return d


@dataclass
class ApplicationListOptions(ListOptions):
    order: t.Optional[Ordering] = None


@dataclass
class EventTypeListOptions(ListOptions):
    with_content: t.Optional[bool] = None
    include_archived: t.Optional[bool] = None


@dataclass
class EndpointListOptions(ListOptions):
    order: t.Optional[Ordering] = None


@dataclass
class EndpointStatsOptions:
    since: t.Optional[datetime] = None
    until: t.Optional[datetime] = None


@dataclass
class IntegrationListOptions(ListOptions):
    pass


class BackgroundTaskListOptions(ListOptions):
    status: t.Optional[BackgroundTaskStatus] = None
    task: t.Optional[BackgroundTaskType] = None


@dataclass
class MessageAttemptListOptions(ListOptions):
    status: t.Optional[MessageStatus] = None
    event_types: t.Optional[t.List[str]] = None
    before: t.Optional[datetime] = None
    after: t.Optional[datetime] = None
    channel: t.Optional[str] = None
    status_code_class: t.Optional[StatusCodeClass] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        d = super().to_dict()
        if self.before is not None:
            d["before"] = ensure_tz(self.before)
        if self.after is not None:
            d["after"] = ensure_tz(self.after)
        return d


class ApiBase:
    _client: AuthenticatedClient

    def __init__(self, client: AuthenticatedClient) -> None:
        self._client = client


class AuthenticationAsync(ApiBase):
    async def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: PostOptions = PostOptions(),
    ) -> AppPortalAccessOut:
        return await v1_authentication_app_portal_access.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=app_portal_access_in,
            **options.to_dict(),
        )

    async def dashboard_access(
        self, app_id: str, options: PostOptions = PostOptions()
    ) -> DashboardAccessOut:
        return await v1_authentication_dashboard_access.request_asyncio(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    async def logout(self, options: PostOptions = PostOptions()) -> None:
        return await v1_authentication_logout.request_asyncio(
            client=self._client, **options.to_dict()
        )


class Authentication(ApiBase):
    def app_portal_access(
        self,
        app_id: str,
        app_portal_access_in: AppPortalAccessIn,
        options: PostOptions = PostOptions(),
    ) -> AppPortalAccessOut:
        return v1_authentication_app_portal_access.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=app_portal_access_in,
            **options.to_dict(),
        )

    def dashboard_access(
        self, app_id: str, options: PostOptions = PostOptions()
    ) -> DashboardAccessOut:
        return v1_authentication_dashboard_access.request_sync(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    def logout(self, options: PostOptions = PostOptions()) -> None:
        return v1_authentication_logout.request_sync(
            client=self._client, **options.to_dict()
        )


class ApplicationAsync(ApiBase):
    async def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        return await v1_application_list.request_asyncio(
            client=self._client, **options.to_dict()
        )

    async def create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return await v1_application_create.request_asyncio(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    async def get(self, app_id: str) -> ApplicationOut:
        return await v1_application_get.request_asyncio(
            client=self._client, app_id=app_id
        )

    async def get_or_create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return await v1_application_create.request_asyncio(
            client=self._client,
            json_body=application_in,
            get_if_exists=True,
            **options.to_dict(),
        )

    async def update(
        self, app_id: str, application_in: ApplicationIn
    ) -> ApplicationOut:
        return await v1_application_update.request_asyncio(
            client=self._client, app_id=app_id, json_body=application_in
        )

    async def patch(
        self, app_id: str, application_patch: ApplicationPatch
    ) -> ApplicationOut:
        return await v1_application_patch.request_asyncio(
            client=self._client, app_id=app_id, json_body=application_patch
        )

    async def delete(self, app_id: str) -> None:
        return await v1_application_delete.request_asyncio(
            client=self._client, app_id=app_id
        )


class Application(ApiBase):
    def list(
        self, options: ApplicationListOptions = ApplicationListOptions()
    ) -> ListResponseApplicationOut:
        return v1_application_list.request_sync(
            client=self._client, **options.to_dict()
        )

    def create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return v1_application_create.request_sync(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    def get(self, app_id: str) -> ApplicationOut:
        return v1_application_get.request_sync(client=self._client, app_id=app_id)

    def get_or_create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return v1_application_create.request_sync(
            client=self._client,
            json_body=application_in,
            get_if_exists=True,
            **options.to_dict(),
        )

    def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        return v1_application_update.request_sync(
            client=self._client, app_id=app_id, json_body=application_in
        )

    def patch(self, app_id: str, application_patch: ApplicationPatch) -> ApplicationOut:
        return v1_application_patch.request_sync(
            client=self._client, app_id=app_id, json_body=application_patch
        )

    def delete(self, app_id: str) -> None:
        return v1_application_delete.request_sync(client=self._client, app_id=app_id)


class EndpointAsync(ApiBase):
    async def list(
        self, app_id: str, options: EndpointListOptions = EndpointListOptions()
    ) -> ListResponseEndpointOut:
        return await v1_endpoint_list.request_asyncio(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    async def create(
        self, app_id: str, endpoint_in: EndpointIn, options: PostOptions = PostOptions()
    ) -> EndpointOut:
        return await v1_endpoint_create.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    async def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        return await v1_endpoint_get.request_asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def update(
        self, app_id: str, endpoint_id: str, endpoint_update: EndpointUpdate
    ) -> EndpointOut:
        return await v1_endpoint_update.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    async def patch(
        self, app_id: str, endpoint_id: str, endpoint_patch: EndpointPatch
    ) -> EndpointOut:
        return await v1_endpoint_patch.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_patch,
        )

    async def delete(self, app_id: str, endpoint_id: str) -> None:
        return await v1_endpoint_delete.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    async def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        return await v1_endpoint_get_secret.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    async def rotate_secret(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: PostOptions = PostOptions(),
    ) -> None:
        return await v1_endpoint_rotate_secret.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_secret_rotate_in,
            **options.to_dict(),
        )

    async def recover(
        self,
        app_id: str,
        endpoint_id: str,
        recover_in: RecoverIn,
        options: PostOptions = PostOptions(),
    ) -> RecoverOut:
        return await v1_endpoint_recover.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=recover_in,
            **options.to_dict(),
        )

    async def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        return await v1_endpoint_get_headers.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    async def update_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn
    ) -> None:
        return await v1_endpoint_update_headers.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    async def patch_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersPatchIn
    ) -> None:
        return await v1_endpoint_patch_headers.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    async def get_stats(
        self,
        app_id: str,
        endpoint_id: str,
        options: EndpointStatsOptions = EndpointStatsOptions(),
    ) -> EndpointStats:
        return await v1_endpoint_get_stats.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            since=ensure_tz(options.since),
            until=ensure_tz(options.until),
        )

    async def replay_missing(
        self,
        app_id: str,
        endpoint_id: str,
        replay_in: ReplayIn,
        options: PostOptions = PostOptions(),
    ) -> ReplayOut:
        return await v1_endpoint_replay.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=replay_in,
            **options.to_dict(),
        )

    async def transformations_get(
        self, app_id: str, endpoint_id: str
    ) -> EndpointTransformationOut:
        return await v1_endpoint_transformation_get.request_asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        await v1_endpoint_transformation_partial_update.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_transformation_in,
        )

    async def send_example(
        self,
        app_id: str,
        endpoint_id: str,
        event_example_in: EventExampleIn,
        options: PostOptions = PostOptions(),
    ) -> MessageOut:
        return await v1_endpoint_send_example.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=event_example_in,
            **options.to_dict(),
        )


class Endpoint(ApiBase):
    def list(
        self, app_id: str, options: EndpointListOptions = EndpointListOptions()
    ) -> ListResponseEndpointOut:
        return v1_endpoint_list.request_sync(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    def create(
        self, app_id: str, endpoint_in: EndpointIn, options: PostOptions = PostOptions()
    ) -> EndpointOut:
        return v1_endpoint_create.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        return v1_endpoint_get.request_sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def update(
        self, app_id: str, endpoint_id: str, endpoint_update: EndpointUpdate
    ) -> EndpointOut:
        return v1_endpoint_update.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    def patch(
        self, app_id: str, endpoint_id: str, endpoint_patch: EndpointPatch
    ) -> EndpointOut:
        return v1_endpoint_patch.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_patch,
        )

    def delete(self, app_id: str, endpoint_id: str) -> None:
        return v1_endpoint_delete.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        return v1_endpoint_get_secret.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    def rotate_secret(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: PostOptions = PostOptions(),
    ) -> None:
        return v1_endpoint_rotate_secret.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_secret_rotate_in,
            **options.to_dict(),
        )

    def recover(
        self,
        app_id: str,
        endpoint_id: str,
        recover_in: RecoverIn,
        options: PostOptions = PostOptions(),
    ) -> RecoverOut:
        return v1_endpoint_recover.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=recover_in,
            **options.to_dict(),
        )

    def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        return v1_endpoint_get_headers.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    def update_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn
    ) -> None:
        return v1_endpoint_update_headers.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    def patch_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersPatchIn
    ) -> None:
        return v1_endpoint_patch_headers.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    def get_stats(
        self,
        app_id: str,
        endpoint_id: str,
        options: EndpointStatsOptions = EndpointStatsOptions(),
    ) -> EndpointStats:
        return v1_endpoint_get_stats.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            since=ensure_tz(options.since),
            until=ensure_tz(options.until),
        )

    def replay_missing(
        self,
        app_id: str,
        endpoint_id: str,
        replay_in: ReplayIn,
        options: PostOptions = PostOptions(),
    ) -> ReplayOut:
        return v1_endpoint_replay.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=replay_in,
            **options.to_dict(),
        )

    def transformations_get(
        self, app_id: str, endpoint_id: str
    ) -> EndpointTransformationOut:
        return v1_endpoint_transformation_get.request_sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        v1_endpoint_transformation_partial_update.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_transformation_in,
        )

    def send_example(
        self,
        app_id: str,
        endpoint_id: str,
        event_example_in: EventExampleIn,
        options: PostOptions = PostOptions(),
    ) -> MessageOut:
        return v1_endpoint_send_example.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=event_example_in,
            **options.to_dict(),
        )


class EventTypeAsync(ApiBase):
    async def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        return await v1_event_type_list.request_asyncio(
            client=self._client,
            **options.to_dict(),
        )

    async def create(
        self, event_type_in: EventTypeIn, options: PostOptions = PostOptions()
    ) -> EventTypeOut:
        return await v1_event_type_create.request_asyncio(
            client=self._client,
            json_body=event_type_in,
            **options.to_dict(),
        )

    async def get(self, event_type_name: str) -> EventTypeOut:
        return await v1_event_type_get.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
        )

    async def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        return await v1_event_type_update.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    async def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        return await v1_event_type_patch.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_patch,
        )

    async def delete(self, event_type_name: str) -> None:
        return await v1_event_type_delete.request_asyncio(
            client=self._client,
            event_type_name=event_type_name,
        )

    async def import_openapi(
        self,
        event_type_import_openapi_in: EventTypeImportOpenApiIn,
        options: PostOptions = PostOptions(),
    ) -> EventTypeImportOpenApiOut:
        return await v1_event_type_import_openapi.request_asyncio(
            client=self._client,
            json_body=event_type_import_openapi_in,
            **options.to_dict(),
        )


class EventType(ApiBase):
    def list(
        self, options: EventTypeListOptions = EventTypeListOptions()
    ) -> ListResponseEventTypeOut:
        return v1_event_type_list.request_sync(
            client=self._client,
            **options.to_dict(),
        )

    def create(
        self, event_type_in: EventTypeIn, options: PostOptions = PostOptions()
    ) -> EventTypeOut:
        return v1_event_type_create.request_sync(
            client=self._client,
            json_body=event_type_in,
            **options.to_dict(),
        )

    def get(self, event_type_name: str) -> EventTypeOut:
        return v1_event_type_get.request_sync(
            client=self._client,
            event_type_name=event_type_name,
        )

    def update(
        self, event_type_name: str, event_type_update: EventTypeUpdate
    ) -> EventTypeOut:
        return v1_event_type_update.request_sync(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    def patch(
        self, event_type_name: str, event_type_patch: EventTypePatch
    ) -> EventTypeOut:
        return v1_event_type_patch.request_sync(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_patch,
        )

    def delete(self, event_type_name: str) -> None:
        return v1_event_type_delete.request_sync(
            client=self._client,
            event_type_name=event_type_name,
        )

    def import_openapi(
        self,
        event_type_import_openapi_in: EventTypeImportOpenApiIn,
        options: PostOptions = PostOptions(),
    ) -> EventTypeImportOpenApiOut:
        return v1_event_type_import_openapi.request_sync(
            client=self._client,
            json_body=event_type_import_openapi_in,
            **options.to_dict(),
        )


class IntegrationAsync(ApiBase):
    async def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        return await v1_integration_list.request_asyncio(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    async def create(
        self, app_id: str, integ_in: IntegrationIn, options: PostOptions = PostOptions()
    ) -> IntegrationOut:
        return await v1_integration_create.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=integ_in,
            **options.to_dict(),
        )

    async def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        return await v1_integration_get.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def update(
        self, app_id: str, integ_id: str, integ_update: IntegrationUpdate
    ) -> IntegrationOut:
        return await v1_integration_update.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integ_update,
        )

    async def delete(self, app_id: str, integ_id: str) -> None:
        return await v1_integration_delete.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return await v1_integration_get_key.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def rotate_key(
        self, app_id: str, integ_id: str, options: PostOptions = PostOptions()
    ) -> IntegrationKeyOut:
        return await v1_integration_rotate_key.request_asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            **options.to_dict(),
        )


class Integration(ApiBase):
    def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        return v1_integration_list.request_sync(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    def create(
        self, app_id: str, integ_in: IntegrationIn, options: PostOptions = PostOptions()
    ) -> IntegrationOut:
        return v1_integration_create.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=integ_in,
            **options.to_dict(),
        )

    def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        return v1_integration_get.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def update(
        self, app_id: str, integ_id: str, integ_update: IntegrationUpdate
    ) -> IntegrationOut:
        return v1_integration_update.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integ_update,
        )

    def delete(self, app_id: str, integ_id: str) -> None:
        return v1_integration_delete.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return v1_integration_get_key.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def rotate_key(
        self, app_id: str, integ_id: str, options: PostOptions = PostOptions()
    ) -> IntegrationKeyOut:
        return v1_integration_rotate_key.request_sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            **options.to_dict(),
        )


class MessageAsync(ApiBase):
    async def list(
        self, app_id: str, options: MessageListOptions = MessageListOptions()
    ) -> ListResponseMessageOut:
        return await v1_message_list.request_asyncio(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    async def create(
        self, app_id: str, message_in: MessageIn, options: PostOptions = PostOptions()
    ) -> MessageOut:
        ret = await v1_message_create.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=message_in,
            with_content=False,
            **options.to_dict(),
        )
        ret.payload = message_in.payload
        return ret

    async def get(self, app_id: str, msg_id: str) -> MessageOut:
        return await v1_message_get.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
        )

    async def expunge_content(self, app_id: str, msg_id: str) -> None:
        return await v1_message_expunge_content.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
        )


class Message(ApiBase):
    def list(
        self, app_id: str, options: MessageListOptions = MessageListOptions()
    ) -> ListResponseMessageOut:
        return v1_message_list.request_sync(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    def create(
        self, app_id: str, message_in: MessageIn, options: PostOptions = PostOptions()
    ) -> MessageOut:
        ret = v1_message_create.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=message_in,
            with_content=False,
            **options.to_dict(),
        )
        ret.payload = message_in.payload
        return ret

    def get(self, app_id: str, msg_id: str) -> MessageOut:
        return v1_message_get.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
        )

    def expunge_content(self, app_id: str, msg_id: str) -> None:
        return v1_message_expunge_content.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
        )


class MessageAttemptAsync(ApiBase):
    async def list_by_msg(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return await v1_message_attempt_list_by_msg.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    async def list_by_endpoint(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return await v1_message_attempt_list_by_endpoint.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        return await v1_message_attempt_get.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )

    async def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: PostOptions = PostOptions(),
    ) -> None:
        return await v1_message_attempt_resend.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def list_attempted_messages(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseEndpointMessageOut:
        return await v1_message_attempt_list_attempted_messages.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def list_attempted_destinations(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageEndpointOut:
        return await v1_message_attempt_list_attempted_destinations.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            **options.to_dict(),
        )

    async def list_attempts_for_endpoint(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptEndpointOut:
        return await v1_message_attempt_list_by_endpoint_deprecated.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def expunge_content(
        self,
        app_id: str,
        msg_id: str,
        attempt_id: str,
    ) -> None:
        return await v1_message_attempt_expunge_content.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )


class MessageAttempt(ApiBase):
    @deprecated(reason="use list_by_msg or list_by_endpoint instead")
    def list(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return self.list_by_msg(app_id=app_id, msg_id=msg_id, options=options)

    def list_by_msg(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return v1_message_attempt_list_by_msg.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    def list_by_endpoint(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return v1_message_attempt_list_by_endpoint.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        return v1_message_attempt_get.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )

    def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: PostOptions = PostOptions(),
    ) -> None:
        return v1_message_attempt_resend.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def list_attempted_messages(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseEndpointMessageOut:
        return v1_message_attempt_list_attempted_messages.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def list_attempted_destinations(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageEndpointOut:
        return v1_message_attempt_list_attempted_destinations.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            **options.to_dict(),
        )

    def list_attempts_for_endpoint(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptEndpointOut:
        return v1_message_attempt_list_by_endpoint_deprecated.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def expunge_content(
        self,
        app_id: str,
        msg_id: str,
        attempt_id: str,
    ) -> None:
        return v1_message_attempt_expunge_content.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )


class BackgroundTaskAsync(ApiBase):
    async def list(
        self, options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ) -> ListResponseBackgroundTaskOut:
        return await list_background_tasks.request_asyncio(
            client=self._client, **options.to_dict()
        )

    async def get(self, task_id: str) -> BackgroundTaskOut:
        return await get_background_task.request_asyncio(
            client=self._client, task_id=task_id
        )


class BackgroundTask(ApiBase):
    def list(
        self, options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ) -> ListResponseBackgroundTaskOut:
        return list_background_tasks.request_sync(
            client=self._client, **options.to_dict()
        )

    def get(self, task_id: str) -> BackgroundTaskOut:
        return get_background_task.request_sync(client=self._client, task_id=task_id)


class StatisticsAsync(ApiBase):
    async def aggregate_app_stats(
        self, app_usage_stats_in: AppUsageStatsIn, options: PostOptions = PostOptions()
    ) -> AppUsageStatsOut:
        return await v1_statistics_aggregate_app_stats.request_asyncio(
            client=self._client,
            json_body=app_usage_stats_in,
            **options.to_dict(),
        )

    async def aggregate_event_types(self, task_id: str) -> AggregateEventTypesOut:
        return await v1_statistics_aggregate_event_types.request_asyncio(
            client=self._client
        )


class Statistics(ApiBase):
    def aggregate_app_stats(
        self, app_usage_stats_in: AppUsageStatsIn, options: PostOptions = PostOptions()
    ) -> AppUsageStatsOut:
        return v1_statistics_aggregate_app_stats.request_sync(
            client=self._client,
            json_body=app_usage_stats_in,
            **options.to_dict(),
        )

    def aggregate_event_types(self, task_id: str) -> AggregateEventTypesOut:
        return v1_statistics_aggregate_event_types.request_sync(client=self._client)


class ClientBase:
    _client: AuthenticatedClient

    def __init__(self, auth_token: str, options: SvixOptions = SvixOptions()) -> None:
        from . import __version__

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
