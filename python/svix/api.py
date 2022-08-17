import typing as t
from dataclasses import asdict, dataclass
from datetime import datetime

from deprecated import deprecated

from .internal.openapi_client.api.application import (
    create_application_api_v1_app_post,
    delete_application_api_v1_app_app_id_delete,
    get_application_api_v1_app_app_id_get,
    list_applications_api_v1_app_get,
    update_application_api_v1_app_app_id_put,
)
from .internal.openapi_client.api.authentication import (
    get_dashboard_access_api_v1_auth_dashboard_access_app_id_post,
    logout_api_v1_auth_logout_post,
)
from .internal.openapi_client.api.endpoint import (
    create_endpoint_api_v1_app_app_id_endpoint_post,
    delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete,
    get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get,
    get_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_get,
    get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get,
    get_endpoint_stats_api_v1_app_app_id_endpoint_endpoint_id_stats_get,
    list_endpoints_api_v1_app_app_id_endpoint_get,
    patch_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_patch,
    recover_failed_webhooks_api_v1_app_app_id_endpoint_endpoint_id_recover_post,
    rotate_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_rotate_post,
    update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put,
    update_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_put,
)
from .internal.openapi_client.api.event_type import (
    create_event_type_api_v1_event_type_post,
    delete_event_type_api_v1_event_type_event_type_name_delete,
    get_event_type_api_v1_event_type_event_type_name_get,
    list_event_types_api_v1_event_type_get,
    update_event_type_api_v1_event_type_event_type_name_put,
)
from .internal.openapi_client.api.integration import (
    create_integration_api_v1_app_app_id_integration_post,
    delete_integration_api_v1_app_app_id_integration_integ_id_delete,
    get_integration_api_v1_app_app_id_integration_integ_id_get,
    get_integration_key_api_v1_app_app_id_integration_integ_id_key_get,
    list_integrations_api_v1_app_app_id_integration_get,
    rotate_integration_key_api_v1_app_app_id_integration_integ_id_key_rotate_post,
    update_integration_api_v1_app_app_id_integration_integ_id_put,
)
from .internal.openapi_client.api.message import (
    create_message_api_v1_app_app_id_msg_post,
    get_message_api_v1_app_app_id_msg_msg_id_get,
    list_messages_api_v1_app_app_id_msg_get,
)
from .internal.openapi_client.api.message_attempt import (
    get_attempt_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_get,
    list_attempted_destinations_api_v1_app_app_id_msg_msg_id_endpoint_get,
    list_attempted_messages_api_v1_app_app_id_endpoint_endpoint_id_msg_get,
    list_attempts_by_endpoint_api_v1_app_app_id_attempt_endpoint_endpoint_id_get,
    list_attempts_by_msg_api_v1_app_app_id_attempt_msg_msg_id_get,
    list_attempts_for_endpoint_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_attempt_get,
    resend_webhook_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_resend_post,
)
from .internal.openapi_client.client import AuthenticatedClient
from .internal.openapi_client.models.application_in import ApplicationIn
from .internal.openapi_client.models.application_out import ApplicationOut
from .internal.openapi_client.models.dashboard_access_out import DashboardAccessOut
from .internal.openapi_client.models.endpoint_headers_in import EndpointHeadersIn
from .internal.openapi_client.models.endpoint_headers_out import EndpointHeadersOut
from .internal.openapi_client.models.endpoint_headers_patch_in import EndpointHeadersPatchIn
from .internal.openapi_client.models.endpoint_in import EndpointIn
from .internal.openapi_client.models.endpoint_message_out_payload import EndpointMessageOutPayload
from .internal.openapi_client.models.endpoint_out import EndpointOut
from .internal.openapi_client.models.endpoint_secret_out import EndpointSecretOut
from .internal.openapi_client.models.endpoint_secret_rotate_in import EndpointSecretRotateIn
from .internal.openapi_client.models.endpoint_stats import EndpointStats
from .internal.openapi_client.models.endpoint_update import EndpointUpdate
from .internal.openapi_client.models.event_type_in import EventTypeIn
from .internal.openapi_client.models.event_type_out import EventTypeOut
from .internal.openapi_client.models.event_type_update import EventTypeUpdate
from .internal.openapi_client.models.integration_in import IntegrationIn
from .internal.openapi_client.models.integration_key_out import IntegrationKeyOut
from .internal.openapi_client.models.integration_out import IntegrationOut
from .internal.openapi_client.models.integration_update import IntegrationUpdate
from .internal.openapi_client.models.list_response_application_out import ListResponseApplicationOut
from .internal.openapi_client.models.list_response_endpoint_message_out import ListResponseEndpointMessageOut
from .internal.openapi_client.models.list_response_endpoint_out import ListResponseEndpointOut
from .internal.openapi_client.models.list_response_event_type_out import ListResponseEventTypeOut
from .internal.openapi_client.models.list_response_integration_out import ListResponseIntegrationOut
from .internal.openapi_client.models.list_response_message_attempt_endpoint_out import (
    ListResponseMessageAttemptEndpointOut,
)
from .internal.openapi_client.models.list_response_message_attempt_out import ListResponseMessageAttemptOut
from .internal.openapi_client.models.list_response_message_endpoint_out import ListResponseMessageEndpointOut
from .internal.openapi_client.models.list_response_message_out import ListResponseMessageOut
from .internal.openapi_client.models.message_attempt_out import MessageAttemptOut
from .internal.openapi_client.models.message_in import MessageIn
from .internal.openapi_client.models.message_in_payload import MessageInPayload
from .internal.openapi_client.models.message_out import MessageOut
from .internal.openapi_client.models.message_out_payload import MessageOutPayload
from .internal.openapi_client.models.message_status import MessageStatus
from .internal.openapi_client.models.recover_in import RecoverIn
from .internal.openapi_client.models.recover_out import RecoverOut
from .internal.openapi_client.models.status_code_class import StatusCodeClass

DEFAULT_SERVER_URL = "https://api.svix.com"


@dataclass
class SvixOptions:
    debug: bool = False
    server_url: t.Optional[str] = None


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


@dataclass
class ApplicationListOptions(ListOptions):
    pass


@dataclass
class EventTypeListOptions(ListOptions):
    with_content: t.Optional[bool] = None
    include_archived: t.Optional[bool] = None


@dataclass
class EndpointListOptions(ListOptions):
    pass


@dataclass
class IntegrationListOptions(ListOptions):
    pass


@dataclass
class MessageAttemptListOptions(ListOptions):
    status: t.Optional[MessageStatus] = None
    event_types: t.Optional[t.List[str]] = None
    before: t.Optional[datetime] = None
    after: t.Optional[datetime] = None
    channel: t.Optional[str] = None
    status_code_class: t.Optional[StatusCodeClass] = None


class ApiBase:
    _client: AuthenticatedClient

    def __init__(self, client: AuthenticatedClient) -> None:
        self._client = client


class AuthenticationAsync(ApiBase):
    async def dashboard_access(self, app_id: str, options: PostOptions = PostOptions()) -> DashboardAccessOut:
        return await get_dashboard_access_api_v1_auth_dashboard_access_app_id_post.asyncio(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    async def logout(self, options: PostOptions = PostOptions()) -> None:
        return await logout_api_v1_auth_logout_post.asyncio(client=self._client, **options.to_dict())


class Authentication(ApiBase):
    def dashboard_access(self, app_id: str, options: PostOptions = PostOptions()) -> DashboardAccessOut:
        return get_dashboard_access_api_v1_auth_dashboard_access_app_id_post.sync(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    def logout(self, options: PostOptions = PostOptions()) -> None:
        return logout_api_v1_auth_logout_post.sync(client=self._client, **options.to_dict())


class ApplicationAsync(ApiBase):
    async def list(self, options: ApplicationListOptions = ApplicationListOptions()) -> ListResponseApplicationOut:
        return await list_applications_api_v1_app_get.asyncio(client=self._client, **options.to_dict())

    async def create(self, application_in: ApplicationIn, options: PostOptions = PostOptions()) -> ApplicationOut:
        return await create_application_api_v1_app_post.asyncio(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    async def get(self, app_id: str) -> ApplicationOut:
        return await get_application_api_v1_app_app_id_get.asyncio(client=self._client, app_id=app_id)

    async def get_or_create(
        self, application_in: ApplicationIn, options: PostOptions = PostOptions()
    ) -> ApplicationOut:
        return await create_application_api_v1_app_post.asyncio(
            client=self._client, json_body=application_in, get_if_exists=True, **options.to_dict()
        )

    async def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        return await update_application_api_v1_app_app_id_put.asyncio(
            client=self._client, app_id=app_id, json_body=application_in
        )

    async def delete(self, app_id: str) -> None:
        return await delete_application_api_v1_app_app_id_delete.asyncio(client=self._client, app_id=app_id)


class Application(ApiBase):
    def list(self, options: ApplicationListOptions = ApplicationListOptions()) -> ListResponseApplicationOut:
        return list_applications_api_v1_app_get.sync(client=self._client, **options.to_dict())

    def create(self, application_in: ApplicationIn, options: PostOptions = PostOptions()) -> ApplicationOut:
        return create_application_api_v1_app_post.sync(
            client=self._client, json_body=application_in, **options.to_dict()
        )

    def get(self, app_id: str) -> ApplicationOut:
        return get_application_api_v1_app_app_id_get.sync(client=self._client, app_id=app_id)

    def get_or_create(self, application_in: ApplicationIn, options: PostOptions = PostOptions()) -> ApplicationOut:
        return create_application_api_v1_app_post.sync(
            client=self._client, json_body=application_in, get_if_exists=True, **options.to_dict()
        )

    def update(self, app_id: str, application_in: ApplicationIn) -> ApplicationOut:
        return update_application_api_v1_app_app_id_put.sync(
            client=self._client, app_id=app_id, json_body=application_in
        )

    def delete(self, app_id: str) -> None:
        return delete_application_api_v1_app_app_id_delete.sync(client=self._client, app_id=app_id)


class EndpointAsync(ApiBase):
    async def list(self, app_id: str, options: EndpointListOptions = EndpointListOptions()) -> ListResponseEndpointOut:
        return await list_endpoints_api_v1_app_app_id_endpoint_get.asyncio(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    async def create(self, app_id: str, endpoint_in: EndpointIn, options: PostOptions = PostOptions()) -> EndpointOut:
        return await create_endpoint_api_v1_app_app_id_endpoint_post.asyncio(
            client=self._client,
            app_id=app_id,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    async def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        return await get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get.asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def update(self, app_id: str, endpoint_id: str, endpoint_update: EndpointUpdate) -> EndpointOut:
        return await update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    async def delete(self, app_id: str, endpoint_id: str) -> None:
        return await delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    async def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        return await get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get.asyncio(
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
        return await rotate_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_rotate_post.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_secret_rotate_in,
            **options.to_dict(),
        )

    async def recover(
        self, app_id: str, endpoint_id: str, recover_in: RecoverIn, options: PostOptions = PostOptions()
    ) -> RecoverOut:
        return await recover_failed_webhooks_api_v1_app_app_id_endpoint_endpoint_id_recover_post.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=recover_in,
            **options.to_dict(),
        )

    async def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        return await get_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_get.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    async def update_headers(self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn) -> None:
        return await update_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_put.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    async def patch_headers(self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersPatchIn) -> None:
        return await patch_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_patch.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )


class Endpoint(ApiBase):
    def list(self, app_id: str, options: EndpointListOptions = EndpointListOptions()) -> ListResponseEndpointOut:
        return list_endpoints_api_v1_app_app_id_endpoint_get.sync(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    def create(self, app_id: str, endpoint_in: EndpointIn, options: PostOptions = PostOptions()) -> EndpointOut:
        return create_endpoint_api_v1_app_app_id_endpoint_post.sync(
            client=self._client,
            app_id=app_id,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        return get_endpoint_api_v1_app_app_id_endpoint_endpoint_id_get.sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def update(self, app_id: str, endpoint_id: str, endpoint_update: EndpointUpdate) -> EndpointOut:
        return update_endpoint_api_v1_app_app_id_endpoint_endpoint_id_put.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    def delete(self, app_id: str, endpoint_id: str) -> None:
        return delete_endpoint_api_v1_app_app_id_endpoint_endpoint_id_delete.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        return get_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_get.sync(
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
        return rotate_endpoint_secret_api_v1_app_app_id_endpoint_endpoint_id_secret_rotate_post.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_secret_rotate_in,
            **options.to_dict(),
        )

    def recover(
        self, app_id: str, endpoint_id: str, recover_in: RecoverIn, options: PostOptions = PostOptions()
    ) -> RecoverOut:
        return recover_failed_webhooks_api_v1_app_app_id_endpoint_endpoint_id_recover_post.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=recover_in,
            **options.to_dict(),
        )

    def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        return get_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_get.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )

    def update_headers(self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn) -> None:
        return update_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_put.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    def patch_headers(self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersPatchIn) -> None:
        return patch_endpoint_headers_api_v1_app_app_id_endpoint_endpoint_id_headers_patch.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    def get_stats(self, app_id: str, endpoint_id: str) -> EndpointStats:
        return get_endpoint_stats_api_v1_app_app_id_endpoint_endpoint_id_stats_get.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
        )


class EventTypeAsync(ApiBase):
    async def list(self, options: EventTypeListOptions = EventTypeListOptions()) -> ListResponseEventTypeOut:
        return await list_event_types_api_v1_event_type_get.asyncio(
            client=self._client,
            **options.to_dict(),
        )

    async def create(self, event_type_in: EventTypeIn, options: PostOptions = PostOptions()) -> EventTypeOut:
        return await create_event_type_api_v1_event_type_post.asyncio(
            client=self._client,
            json_body=event_type_in,
            **options.to_dict(),
        )

    async def get(self, event_type_name: str) -> EventTypeOut:
        return await get_event_type_api_v1_event_type_event_type_name_get.asyncio(
            client=self._client,
            event_type_name=event_type_name,
        )

    async def update(self, event_type_name: str, event_type_update: EventTypeUpdate) -> EventTypeOut:
        return await update_event_type_api_v1_event_type_event_type_name_put.asyncio(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    async def delete(self, event_type_name: str) -> None:
        return await delete_event_type_api_v1_event_type_event_type_name_delete.asyncio(
            client=self._client,
            event_type_name=event_type_name,
        )


class EventType(ApiBase):
    def list(self, options: EventTypeListOptions = EventTypeListOptions()) -> ListResponseEventTypeOut:
        return list_event_types_api_v1_event_type_get.sync(
            client=self._client,
            **options.to_dict(),
        )

    def create(self, event_type_in: EventTypeIn, options: PostOptions = PostOptions()) -> EventTypeOut:
        return create_event_type_api_v1_event_type_post.sync(
            client=self._client,
            json_body=event_type_in,
            **options.to_dict(),
        )

    def get(self, event_type_name: str) -> EventTypeOut:
        return get_event_type_api_v1_event_type_event_type_name_get.sync(
            client=self._client,
            event_type_name=event_type_name,
        )

    def update(self, event_type_name: str, event_type_update: EventTypeUpdate) -> EventTypeOut:
        return update_event_type_api_v1_event_type_event_type_name_put.sync(
            client=self._client,
            event_type_name=event_type_name,
            json_body=event_type_update,
        )

    def delete(self, event_type_name: str) -> None:
        return delete_event_type_api_v1_event_type_event_type_name_delete.sync(
            client=self._client,
            event_type_name=event_type_name,
        )


class IntegrationAsync(ApiBase):
    async def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        return await list_integrations_api_v1_app_app_id_integration_get.asyncio(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    async def create(
        self, app_id: str, integ_in: IntegrationIn, options: PostOptions = PostOptions()
    ) -> IntegrationOut:
        return await create_integration_api_v1_app_app_id_integration_post.asyncio(
            client=self._client,
            app_id=app_id,
            json_body=integ_in,
            **options.to_dict(),
        )

    async def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        return await get_integration_api_v1_app_app_id_integration_integ_id_get.asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def update(self, app_id: str, integ_id: str, integ_update: IntegrationUpdate) -> IntegrationOut:
        return await update_integration_api_v1_app_app_id_integration_integ_id_put.asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integ_update,
        )

    async def delete(self, app_id: str, integ_id: str) -> None:
        return await delete_integration_api_v1_app_app_id_integration_integ_id_delete.asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return await get_integration_key_api_v1_app_app_id_integration_integ_id_key_get.asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    async def rotate_key(self, app_id: str, integ_id: str, options: PostOptions = PostOptions()) -> IntegrationKeyOut:
        return await rotate_integration_key_api_v1_app_app_id_integration_integ_id_key_rotate_post.asyncio(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            **options.to_dict(),
        )


class Integration(ApiBase):
    def list(
        self, app_id: str, options: IntegrationListOptions = IntegrationListOptions()
    ) -> ListResponseIntegrationOut:
        return list_integrations_api_v1_app_app_id_integration_get.sync(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    def create(self, app_id: str, integ_in: IntegrationIn, options: PostOptions = PostOptions()) -> IntegrationOut:
        return create_integration_api_v1_app_app_id_integration_post.sync(
            client=self._client,
            app_id=app_id,
            json_body=integ_in,
            **options.to_dict(),
        )

    def get(self, app_id: str, integ_id: str) -> IntegrationOut:
        return get_integration_api_v1_app_app_id_integration_integ_id_get.sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def update(self, app_id: str, integ_id: str, integ_update: IntegrationUpdate) -> IntegrationOut:
        return update_integration_api_v1_app_app_id_integration_integ_id_put.sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            json_body=integ_update,
        )

    def delete(self, app_id: str, integ_id: str) -> None:
        return delete_integration_api_v1_app_app_id_integration_integ_id_delete.sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def get_key(self, app_id: str, integ_id: str) -> IntegrationKeyOut:
        return get_integration_key_api_v1_app_app_id_integration_integ_id_key_get.sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
        )

    def rotate_key(self, app_id: str, integ_id: str, options: PostOptions = PostOptions()) -> IntegrationKeyOut:
        return rotate_integration_key_api_v1_app_app_id_integration_integ_id_key_rotate_post.sync(
            client=self._client,
            app_id=app_id,
            integ_id=integ_id,
            **options.to_dict(),
        )


class MessageAsync(ApiBase):
    async def list(self, app_id: str, options: MessageListOptions = MessageListOptions()) -> ListResponseMessageOut:
        return await list_messages_api_v1_app_app_id_msg_get.asyncio(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    async def create(self, app_id: str, message_in: MessageIn, options: PostOptions = PostOptions()) -> MessageOut:
        ret = await create_message_api_v1_app_app_id_msg_post.asyncio(
            client=self._client,
            app_id=app_id,
            json_body=message_in,
            with_content=False,
            **options.to_dict(),
        )
        ret.payload = message_in.payload
        return ret

    async def get(self, app_id: str, msg_id: str) -> MessageOut:
        return await get_message_api_v1_app_app_id_msg_msg_id_get.asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
        )


class Message(ApiBase):
    def list(self, app_id: str, options: MessageListOptions = MessageListOptions()) -> ListResponseMessageOut:
        return list_messages_api_v1_app_app_id_msg_get.sync(
            client=self._client,
            app_id=app_id,
            **options.to_dict(),
        )

    def create(self, app_id: str, message_in: MessageIn, options: PostOptions = PostOptions()) -> MessageOut:
        ret = create_message_api_v1_app_app_id_msg_post.sync(
            client=self._client,
            app_id=app_id,
            json_body=message_in,
            with_content=False,
            **options.to_dict(),
        )
        ret.payload = message_in.payload
        return ret

    def get(self, app_id: str, msg_id: str) -> MessageOut:
        return get_message_api_v1_app_app_id_msg_msg_id_get.sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
        )


class MessageAttemptAsync(ApiBase):
    async def list_by_msg(
        self, app_id: str, msg_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageAttemptOut:
        return await list_attempts_by_msg_api_v1_app_app_id_attempt_msg_msg_id_get.asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    async def list_by_endpoint(
        self, app_id: str, endpoint_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageAttemptOut:
        return await list_attempts_by_endpoint_api_v1_app_app_id_attempt_endpoint_endpoint_id_get.asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id, **options.to_dict()
        )

    async def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        return await get_attempt_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_get.asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )

    async def resend(self, app_id: str, msg_id: str, endpoint_id: str, options: PostOptions = PostOptions()) -> None:
        return await resend_webhook_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_resend_post.asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def list_attempted_messages(
        self, app_id: str, endpoint_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseEndpointMessageOut:
        return await list_attempted_messages_api_v1_app_app_id_endpoint_endpoint_id_msg_get.asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def list_attempted_destinations(
        self, app_id: str, msg_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageEndpointOut:
        return await list_attempted_destinations_api_v1_app_app_id_msg_msg_id_endpoint_get.asyncio(
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
        return await list_attempts_for_endpoint_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_attempt_get.asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )


class MessageAttempt(ApiBase):
    @deprecated(reason="use list_by_msg or list_by_endpoint instead")
    def list(
        self, app_id: str, msg_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageAttemptOut:
        return self.list_by_msg(app_id=app_id, msg_id=msg_id, options=options)

    def list_by_msg(
        self, app_id: str, msg_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageAttemptOut:
        return list_attempts_by_msg_api_v1_app_app_id_attempt_msg_msg_id_get.sync(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    def list_by_endpoint(
        self, app_id: str, endpoint_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageAttemptOut:
        return list_attempts_by_endpoint_api_v1_app_app_id_attempt_endpoint_endpoint_id_get.sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id, **options.to_dict()
        )

    def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        return get_attempt_api_v1_app_app_id_msg_msg_id_attempt_attempt_id_get.sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )

    def resend(self, app_id: str, msg_id: str, endpoint_id: str, options: PostOptions = PostOptions()) -> None:
        return resend_webhook_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_resend_post.sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def list_attempted_messages(
        self, app_id: str, endpoint_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseEndpointMessageOut:
        return list_attempted_messages_api_v1_app_app_id_endpoint_endpoint_id_msg_get.sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def list_attempted_destinations(
        self, app_id: str, msg_id: str, options: MessageAttemptListOptions = MessageAttemptListOptions()
    ) -> ListResponseMessageEndpointOut:
        return list_attempted_destinations_api_v1_app_app_id_msg_msg_id_endpoint_get.sync(
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
        return list_attempts_for_endpoint_api_v1_app_app_id_msg_msg_id_endpoint_endpoint_id_attempt_get.sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )


class ClientBase:
    _client: AuthenticatedClient

    def __init__(self, auth_token: str, options: SvixOptions = SvixOptions()) -> None:
        from . import __version__

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
            timeout=15,
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


__all__ = [
    "ApplicationIn",
    "ApplicationOut",
    "ListResponseApplicationOut",
    "DashboardAccessOut",
    "EndpointHeadersIn",
    "EndpointHeadersPatchIn",
    "EndpointHeadersOut",
    "EndpointIn",
    "EndpointOut",
    "EndpointSecretOut",
    "EndpointSecretRotateIn",
    "ListResponseEndpointOut",
    "EventTypeIn",
    "EventTypeOut",
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
