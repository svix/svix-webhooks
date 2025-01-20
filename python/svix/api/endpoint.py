import typing as t
from datetime import datetime
from dataclasses import dataclass

from .common import ApiBase, BaseOptions
from ..internal.openapi_client import models


from ..internal.openapi_client.api.endpoint import (
    v1_endpoint_list,
    v1_endpoint_create,
    v1_endpoint_get,
    v1_endpoint_update,
    v1_endpoint_delete,
    v1_endpoint_patch,
    v1_endpoint_get_headers,
    v1_endpoint_update_headers,
    v1_endpoint_patch_headers,
    v1_endpoint_recover,
    v1_endpoint_replay_missing,
    v1_endpoint_get_secret,
    v1_endpoint_rotate_secret,
    v1_endpoint_send_example,
    v1_endpoint_get_stats,
    v1_endpoint_transformation_get,
    v1_endpoint_transformation_partial_update,
)

from ..internal.openapi_client.models.list_response_endpoint_out import (
    ListResponseEndpointOut,
)
from ..internal.openapi_client.models.endpoint_in import EndpointIn
from ..internal.openapi_client.models.endpoint_out import EndpointOut
from ..internal.openapi_client.models.endpoint_update import EndpointUpdate
from ..internal.openapi_client.models.endpoint_patch import EndpointPatch
from ..internal.openapi_client.models.endpoint_headers_out import EndpointHeadersOut
from ..internal.openapi_client.models.endpoint_headers_in import EndpointHeadersIn
from ..internal.openapi_client.models.endpoint_headers_patch_in import (
    EndpointHeadersPatchIn,
)
from ..internal.openapi_client.models.recover_in import RecoverIn
from ..internal.openapi_client.models.recover_out import RecoverOut
from ..internal.openapi_client.models.replay_in import ReplayIn
from ..internal.openapi_client.models.replay_out import ReplayOut
from ..internal.openapi_client.models.endpoint_secret_out import EndpointSecretOut
from ..internal.openapi_client.models.endpoint_secret_rotate_in import (
    EndpointSecretRotateIn,
)
from ..internal.openapi_client.models.event_example_in import EventExampleIn
from ..internal.openapi_client.models.message_out import MessageOut
from ..internal.openapi_client.models.endpoint_stats import EndpointStats
from ..internal.openapi_client.models.endpoint_transformation_out import (
    EndpointTransformationOut,
)
from ..internal.openapi_client.models.endpoint_transformation_in import (
    EndpointTransformationIn,
)


@dataclass
class EndpointListOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # The sorting order of the returned items
    order: t.Optional[models.Ordering] = None


@dataclass
class EndpointCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class EndpointRecoverOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class EndpointReplayMissingOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class EndpointRotateSecretOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class EndpointSendExampleOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class EndpointGetStatsOptions(BaseOptions):
    # Filter the range to data starting from this date.
    since: t.Optional[datetime] = None
    # Filter the range to data ending by this date.
    until: t.Optional[datetime] = None


class EndpointAsync(ApiBase):
    async def list(
        self, app_id: str, options: EndpointListOptions = EndpointListOptions()
    ) -> ListResponseEndpointOut:
        """List the application's endpoints."""
        return await v1_endpoint_list.request_asyncio(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    async def create(
        self,
        app_id: str,
        endpoint_in: EndpointIn,
        options: EndpointCreateOptions = EndpointCreateOptions(),
    ) -> EndpointOut:
        """Create a new endpoint for the application.

        When `secret` is `null` the secret is automatically generated (recommended)."""
        return await v1_endpoint_create.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    async def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        """Get an endpoint."""
        return await v1_endpoint_get.request_asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def update(
        self, app_id: str, endpoint_id: str, endpoint_update: EndpointUpdate
    ) -> EndpointOut:
        """Update an endpoint."""
        return await v1_endpoint_update.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    async def delete(self, app_id: str, endpoint_id: str) -> None:
        """Delete an endpoint."""
        return await v1_endpoint_delete.request_asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def patch(
        self, app_id: str, endpoint_id: str, endpoint_patch: EndpointPatch
    ) -> EndpointOut:
        """Partially update an endpoint."""
        return await v1_endpoint_patch.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_patch,
        )

    async def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        """Get the additional headers to be sent with the webhook."""
        return await v1_endpoint_get_headers.request_asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def update_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn
    ) -> None:
        """Set the additional headers to be sent with the webhook."""
        return await v1_endpoint_update_headers.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    async def patch_headers(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_headers_patch_in: EndpointHeadersPatchIn,
    ) -> None:
        """Partially set the additional headers to be sent with the webhook."""
        return await v1_endpoint_patch_headers.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_patch_in,
        )

    async def recover(
        self,
        app_id: str,
        endpoint_id: str,
        recover_in: RecoverIn,
        options: EndpointRecoverOptions = EndpointRecoverOptions(),
    ) -> RecoverOut:
        """Resend all failed messages since a given time.

        Messages that were sent successfully, even if failed initially, are not resent."""
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
        options: EndpointReplayMissingOptions = EndpointReplayMissingOptions(),
    ) -> ReplayOut:
        """Replays messages to the endpoint.

        Only messages that were created after `since` will be sent.
        Messages that were previously sent to the endpoint are not resent."""
        return await v1_endpoint_replay_missing.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=replay_in,
            **options.to_dict(),
        )

    async def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        """Get the endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        return await v1_endpoint_get_secret.request_asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def rotate_secret(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: EndpointRotateSecretOptions = EndpointRotateSecretOptions(),
    ) -> None:
        """Rotates the endpoint's signing secret.

        The previous secret will remain valid for the next 24 hours."""
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
        options: EndpointSendExampleOptions = EndpointSendExampleOptions(),
    ) -> MessageOut:
        """Send an example message for an event."""
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
        options: EndpointGetStatsOptions = EndpointGetStatsOptions(),
    ) -> EndpointStats:
        """Get basic statistics for the endpoint."""
        return await v1_endpoint_get_stats.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def transformation_get(
        self, app_id: str, endpoint_id: str
    ) -> EndpointTransformationOut:
        """Get the transformation code associated with this endpoint."""
        return await v1_endpoint_transformation_get.request_asyncio(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    async def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        """Set or unset the transformation code associated with this endpoint."""
        return await v1_endpoint_transformation_partial_update.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_transformation_in,
        )


class Endpoint(ApiBase):
    def list(
        self, app_id: str, options: EndpointListOptions = EndpointListOptions()
    ) -> ListResponseEndpointOut:
        """List the application's endpoints."""
        return v1_endpoint_list.request_sync(
            client=self._client, app_id=app_id, **options.to_dict()
        )

    def create(
        self,
        app_id: str,
        endpoint_in: EndpointIn,
        options: EndpointCreateOptions = EndpointCreateOptions(),
    ) -> EndpointOut:
        """Create a new endpoint for the application.

        When `secret` is `null` the secret is automatically generated (recommended)."""
        return v1_endpoint_create.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=endpoint_in,
            **options.to_dict(),
        )

    def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        """Get an endpoint."""
        return v1_endpoint_get.request_sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def update(
        self, app_id: str, endpoint_id: str, endpoint_update: EndpointUpdate
    ) -> EndpointOut:
        """Update an endpoint."""
        return v1_endpoint_update.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_update,
        )

    def delete(self, app_id: str, endpoint_id: str) -> None:
        """Delete an endpoint."""
        return v1_endpoint_delete.request_sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def patch(
        self, app_id: str, endpoint_id: str, endpoint_patch: EndpointPatch
    ) -> EndpointOut:
        """Partially update an endpoint."""
        return v1_endpoint_patch.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_patch,
        )

    def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        """Get the additional headers to be sent with the webhook."""
        return v1_endpoint_get_headers.request_sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def update_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn
    ) -> None:
        """Set the additional headers to be sent with the webhook."""
        return v1_endpoint_update_headers.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_in,
        )

    def patch_headers(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_headers_patch_in: EndpointHeadersPatchIn,
    ) -> None:
        """Partially set the additional headers to be sent with the webhook."""
        return v1_endpoint_patch_headers.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_headers_patch_in,
        )

    def recover(
        self,
        app_id: str,
        endpoint_id: str,
        recover_in: RecoverIn,
        options: EndpointRecoverOptions = EndpointRecoverOptions(),
    ) -> RecoverOut:
        """Resend all failed messages since a given time.

        Messages that were sent successfully, even if failed initially, are not resent."""
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
        options: EndpointReplayMissingOptions = EndpointReplayMissingOptions(),
    ) -> ReplayOut:
        """Replays messages to the endpoint.

        Only messages that were created after `since` will be sent.
        Messages that were previously sent to the endpoint are not resent."""
        return v1_endpoint_replay_missing.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=replay_in,
            **options.to_dict(),
        )

    def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        """Get the endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        return v1_endpoint_get_secret.request_sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def rotate_secret(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: EndpointRotateSecretOptions = EndpointRotateSecretOptions(),
    ) -> None:
        """Rotates the endpoint's signing secret.

        The previous secret will remain valid for the next 24 hours."""
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
        options: EndpointSendExampleOptions = EndpointSendExampleOptions(),
    ) -> MessageOut:
        """Send an example message for an event."""
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
        options: EndpointGetStatsOptions = EndpointGetStatsOptions(),
    ) -> EndpointStats:
        """Get basic statistics for the endpoint."""
        return v1_endpoint_get_stats.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def transformation_get(
        self, app_id: str, endpoint_id: str
    ) -> EndpointTransformationOut:
        """Get the transformation code associated with this endpoint."""
        return v1_endpoint_transformation_get.request_sync(
            client=self._client, app_id=app_id, endpoint_id=endpoint_id
        )

    def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        """Set or unset the transformation code associated with this endpoint."""
        return v1_endpoint_transformation_partial_update.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            json_body=endpoint_transformation_in,
        )
