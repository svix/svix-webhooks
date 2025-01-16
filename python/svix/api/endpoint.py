import typing as t
from dataclasses import dataclass
from datetime import datetime


from ..internal.openapi_client.api.endpoint import (
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
    v1_endpoint_replay_missing,
    v1_endpoint_rotate_secret,
    v1_endpoint_send_example,
    v1_endpoint_transformation_get,
    v1_endpoint_transformation_partial_update,
    v1_endpoint_update,
    v1_endpoint_update_headers,
)
from ..internal.openapi_client.models.endpoint_headers_in import EndpointHeadersIn
from ..internal.openapi_client.models.endpoint_headers_out import EndpointHeadersOut
from ..internal.openapi_client.models.endpoint_headers_patch_in import (
    EndpointHeadersPatchIn,
)
from ..internal.openapi_client.models.endpoint_in import EndpointIn
from ..internal.openapi_client.models.endpoint_out import EndpointOut
from ..internal.openapi_client.models.endpoint_patch import EndpointPatch
from ..internal.openapi_client.models.endpoint_secret_out import EndpointSecretOut
from ..internal.openapi_client.models.endpoint_secret_rotate_in import (
    EndpointSecretRotateIn,
)
from ..internal.openapi_client.models.endpoint_stats import EndpointStats
from ..internal.openapi_client.models.endpoint_transformation_in import (
    EndpointTransformationIn,
)
from ..internal.openapi_client.models.endpoint_transformation_out import (
    EndpointTransformationOut,
)
from ..internal.openapi_client.models.endpoint_update import EndpointUpdate
from ..internal.openapi_client.models.event_example_in import EventExampleIn
from ..internal.openapi_client.models.list_response_endpoint_out import (
    ListResponseEndpointOut,
)
from ..internal.openapi_client.models.message_out import MessageOut
from ..internal.openapi_client.models.ordering import Ordering
from ..internal.openapi_client.models.recover_in import RecoverIn
from ..internal.openapi_client.models.recover_out import RecoverOut
from ..internal.openapi_client.models.replay_in import ReplayIn
from ..internal.openapi_client.models.replay_out import ReplayOut

from .common import ensure_tz, ListOptions, PostOptions, ApiBase


@dataclass
class EndpointListOptions(ListOptions):
    order: t.Optional[Ordering] = None


@dataclass
class EndpointStatsOptions:
    since: t.Optional[datetime] = None
    until: t.Optional[datetime] = None


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

    async def delete(self, app_id: str, endpoint_id: str) -> None:
        return await v1_endpoint_delete.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
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

    async def replay_missing(
        self,
        app_id: str,
        endpoint_id: str,
        replay_in: ReplayIn,
        options: PostOptions = PostOptions(),
    ) -> ReplayOut:
        return await v1_endpoint_replay_missing.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=replay_in,
            **options.to_dict(),
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

    def delete(self, app_id: str, endpoint_id: str) -> None:
        return v1_endpoint_delete.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
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

    def replay_missing(
        self,
        app_id: str,
        endpoint_id: str,
        replay_in: ReplayIn,
        options: PostOptions = PostOptions(),
    ) -> ReplayOut:
        return v1_endpoint_replay_missing.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=replay_in,
            **options.to_dict(),
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
