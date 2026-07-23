# This file is @generated
import typing as t
from dataclasses import dataclass
from datetime import datetime

from deprecated import deprecated

from .. import models
from ..models import (
    BulkReplayIn,
    EndpointHeadersIn,
    EndpointHeadersOut,
    EndpointHeadersPatchIn,
    EndpointIn,
    EndpointOut,
    EndpointPatch,
    EndpointSecretOut,
    EndpointSecretRotateIn,
    EndpointStats,
    EndpointTransformationIn,
    EndpointUpsertIn,
    EventExampleIn,
    ListResponseEndpointOut,
    MessageOut,
    RecoverIn,
    RecoverOut,
    ReplayIn,
    ReplayOut,
)
from .common import ApiBaseAsync, ApiBaseSync, BaseOptions, serialize_params
from .endpoint_transformation import (
    EndpointTransformation,
    EndpointTransformationAsync,
)


@dataclass
class EndpointListOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
            }
        )


@dataclass
class EndpointCreateOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EndpointRotateSecretOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EndpointReplayMissingOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EndpointBulkReplayOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EndpointGetStatsOptions(BaseOptions):
    since: t.Optional[datetime] = None
    """Filter the range to data starting from this date."""
    until: t.Optional[datetime] = None
    """Filter the range to data ending by this date."""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "since": self.since,
                "until": self.until,
            }
        )


@dataclass
class EndpointRecoverOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class EndpointSendExampleOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class EndpointAsync(ApiBaseAsync):
    @property
    def transformation(self) -> EndpointTransformationAsync:
        return EndpointTransformationAsync(self._client, self._httpx_client)

    async def list(
        self, app_id: str, options: EndpointListOptions = (EndpointListOptions())
    ) -> ListResponseEndpointOut:
        """List the application's endpoints."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/endpoint",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseEndpointOut.model_validate(response.json())

    async def create(
        self,
        app_id: str,
        endpoint_in: EndpointIn,
        options: EndpointCreateOptions = (EndpointCreateOptions()),
    ) -> EndpointOut:
        """Create a new endpoint for the application.

        When `secret` is `null` the secret is automatically generated (recommended)."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/endpoint",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=endpoint_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())

    async def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        """Get an endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointOut.model_validate(response.json())

    async def upsert(
        self, app_id: str, endpoint_id: str, endpoint_upsert_in: EndpointUpsertIn
    ) -> EndpointOut:
        """Create or update an endpoint."""
        response = await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_upsert_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EndpointOut.model_validate(response.json())

    async def delete(self, app_id: str, endpoint_id: str) -> None:
        """Delete an endpoint."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )

    async def patch(
        self, app_id: str, endpoint_id: str, endpoint_patch: EndpointPatch
    ) -> EndpointOut:
        """Partially update an endpoint."""
        response = await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_patch.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())

    async def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        """Get the endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointSecretOut.model_validate(response.json())

    async def rotate_secret(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: EndpointRotateSecretOptions = (EndpointRotateSecretOptions()),
    ) -> None:
        """Rotates the endpoint's signing secret.

        The previous secret will remain valid for the specified grace period (default 24 hours)."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=endpoint_secret_rotate_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    async def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        """Get the additional headers to be sent with the webhook."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointHeadersOut.model_validate(response.json())

    async def set_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn
    ) -> None:
        """Set the additional headers to be sent with the webhook."""
        await self._request_asyncio(
            method="put",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_headers_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    async def patch_headers(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_headers_patch_in: EndpointHeadersPatchIn,
    ) -> None:
        """Partially set the additional headers to be sent with the webhook."""
        await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_headers_patch_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    async def replay_missing(
        self,
        app_id: str,
        endpoint_id: str,
        replay_in: ReplayIn,
        options: EndpointReplayMissingOptions = (EndpointReplayMissingOptions()),
    ) -> ReplayOut:
        """Replays messages to the endpoint.

        Only messages that were created after `since` will be sent.
        Messages that were previously sent to the endpoint are not resent.

        A completed task will return a payload like the following:
        ```json
        {
          "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
          "status": "finished",
          "task": "endpoint.replay",
          "data": {
            "messagesSent": 2
          }
        }
        ```"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=replay_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ReplayOut.model_validate(response.json())

    async def bulk_replay(
        self,
        app_id: str,
        endpoint_id: str,
        bulk_replay_in: BulkReplayIn,
        options: EndpointBulkReplayOptions = (EndpointBulkReplayOptions()),
    ) -> ReplayOut:
        """Bulk replay messages sent to the endpoint.

        Only messages that were created after `since` will be sent.
        This will replay both successful, and failed messages

        A completed task will return a payload like the following:
        ```json
        {
          "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
          "status": "finished",
          "task": "endpoint.bulk-replay",
          "data": {
            "messagesSent": 2
          }
        }
        ```"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/bulk-replay",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=bulk_replay_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ReplayOut.model_validate(response.json())

    async def get_stats(
        self,
        app_id: str,
        endpoint_id: str,
        options: EndpointGetStatsOptions = (EndpointGetStatsOptions()),
    ) -> EndpointStats:
        """Get basic statistics for the endpoint."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return EndpointStats.model_validate(response.json())

    async def recover(
        self,
        app_id: str,
        endpoint_id: str,
        recover_in: RecoverIn,
        options: EndpointRecoverOptions = (EndpointRecoverOptions()),
    ) -> RecoverOut:
        """Resend all failed messages since a given time.

        Messages that were sent successfully, even if failed initially, are not resent.

        A completed task will return a payload like the following:
        ```json
        {
          "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
          "status": "finished",
          "task": "endpoint.recover",
          "data": {
            "messagesSent": 2
          }
        }
        ```"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=recover_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return RecoverOut.model_validate(response.json())

    async def send_example(
        self,
        app_id: str,
        endpoint_id: str,
        event_example_in: EventExampleIn,
        options: EndpointSendExampleOptions = (EndpointSendExampleOptions()),
    ) -> MessageOut:
        """Send an example message for an event."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=event_example_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return MessageOut.model_validate(response.json())

    @deprecated
    async def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        """This operation was renamed to `set-transformation`."""
        await self._request_asyncio(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_transformation_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class Endpoint(ApiBaseSync):
    @property
    def transformation(self) -> EndpointTransformation:
        return EndpointTransformation(self._client, self._httpx_client)

    def list(
        self, app_id: str, options: EndpointListOptions = (EndpointListOptions())
    ) -> ListResponseEndpointOut:
        """List the application's endpoints."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/endpoint",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseEndpointOut.model_validate(response.json())

    def create(
        self,
        app_id: str,
        endpoint_in: EndpointIn,
        options: EndpointCreateOptions = (EndpointCreateOptions()),
    ) -> EndpointOut:
        """Create a new endpoint for the application.

        When `secret` is `null` the secret is automatically generated (recommended)."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/endpoint",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=endpoint_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())

    def get(self, app_id: str, endpoint_id: str) -> EndpointOut:
        """Get an endpoint."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointOut.model_validate(response.json())

    def upsert(
        self, app_id: str, endpoint_id: str, endpoint_upsert_in: EndpointUpsertIn
    ) -> EndpointOut:
        """Create or update an endpoint."""
        response = self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_upsert_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return EndpointOut.model_validate(response.json())

    def delete(self, app_id: str, endpoint_id: str) -> None:
        """Delete an endpoint."""
        self._request_sync(
            method="delete",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )

    def patch(
        self, app_id: str, endpoint_id: str, endpoint_patch: EndpointPatch
    ) -> EndpointOut:
        """Partially update an endpoint."""
        response = self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_patch.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return EndpointOut.model_validate(response.json())

    def get_secret(self, app_id: str, endpoint_id: str) -> EndpointSecretOut:
        """Get the endpoint's signing secret.

        This is used to verify the authenticity of the webhook.
        For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/)."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointSecretOut.model_validate(response.json())

    def rotate_secret(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_secret_rotate_in: EndpointSecretRotateIn,
        options: EndpointRotateSecretOptions = (EndpointRotateSecretOptions()),
    ) -> None:
        """Rotates the endpoint's signing secret.

        The previous secret will remain valid for the specified grace period (default 24 hours)."""
        self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=endpoint_secret_rotate_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    def get_headers(self, app_id: str, endpoint_id: str) -> EndpointHeadersOut:
        """Get the additional headers to be sent with the webhook."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
        )
        return EndpointHeadersOut.model_validate(response.json())

    def set_headers(
        self, app_id: str, endpoint_id: str, endpoint_headers_in: EndpointHeadersIn
    ) -> None:
        """Set the additional headers to be sent with the webhook."""
        self._request_sync(
            method="put",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_headers_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    def patch_headers(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_headers_patch_in: EndpointHeadersPatchIn,
    ) -> None:
        """Partially set the additional headers to be sent with the webhook."""
        self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_headers_patch_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )

    def replay_missing(
        self,
        app_id: str,
        endpoint_id: str,
        replay_in: ReplayIn,
        options: EndpointReplayMissingOptions = (EndpointReplayMissingOptions()),
    ) -> ReplayOut:
        """Replays messages to the endpoint.

        Only messages that were created after `since` will be sent.
        Messages that were previously sent to the endpoint are not resent.

        A completed task will return a payload like the following:
        ```json
        {
          "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
          "status": "finished",
          "task": "endpoint.replay",
          "data": {
            "messagesSent": 2
          }
        }
        ```"""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=replay_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ReplayOut.model_validate(response.json())

    def bulk_replay(
        self,
        app_id: str,
        endpoint_id: str,
        bulk_replay_in: BulkReplayIn,
        options: EndpointBulkReplayOptions = (EndpointBulkReplayOptions()),
    ) -> ReplayOut:
        """Bulk replay messages sent to the endpoint.

        Only messages that were created after `since` will be sent.
        This will replay both successful, and failed messages

        A completed task will return a payload like the following:
        ```json
        {
          "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
          "status": "finished",
          "task": "endpoint.bulk-replay",
          "data": {
            "messagesSent": 2
          }
        }
        ```"""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/bulk-replay",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=bulk_replay_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return ReplayOut.model_validate(response.json())

    def get_stats(
        self,
        app_id: str,
        endpoint_id: str,
        options: EndpointGetStatsOptions = (EndpointGetStatsOptions()),
    ) -> EndpointStats:
        """Get basic statistics for the endpoint."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return EndpointStats.model_validate(response.json())

    def recover(
        self,
        app_id: str,
        endpoint_id: str,
        recover_in: RecoverIn,
        options: EndpointRecoverOptions = (EndpointRecoverOptions()),
    ) -> RecoverOut:
        """Resend all failed messages since a given time.

        Messages that were sent successfully, even if failed initially, are not resent.

        A completed task will return a payload like the following:
        ```json
        {
          "id": "qtask_33qen93MNuelBAq1T9G7eHLJRsF",
          "status": "finished",
          "task": "endpoint.recover",
          "data": {
            "messagesSent": 2
          }
        }
        ```"""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=recover_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return RecoverOut.model_validate(response.json())

    def send_example(
        self,
        app_id: str,
        endpoint_id: str,
        event_example_in: EventExampleIn,
        options: EndpointSendExampleOptions = (EndpointSendExampleOptions()),
    ) -> MessageOut:
        """Send an example message for an event."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=event_example_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return MessageOut.model_validate(response.json())

    @deprecated
    def transformation_partial_update(
        self,
        app_id: str,
        endpoint_id: str,
        endpoint_transformation_in: EndpointTransformationIn,
    ) -> None:
        """This operation was renamed to `set-transformation`."""
        self._request_sync(
            method="patch",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            json_body=endpoint_transformation_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
