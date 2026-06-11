# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import PollerV2CommitIn, PollerV2PollOut
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class MessagePollerv2ConsumerPollOptions(BaseOptions):
    limit: t.Optional[int] = None
    lease_duration_ms: t.Optional[int] = None
    """Lease duration in milliseconds."""
    starting_position: t.Optional[models.StartingPosition] = None

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "lease_duration_ms": self.lease_duration_ms,
                "starting_position": self.starting_position,
            }
        )


@dataclass
class MessagePollerv2ConsumerCommitOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class MessagePollerv2Async(ApiBase):
    async def consumer_poll(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        options: MessagePollerv2ConsumerPollOptions = (
            MessagePollerv2ConsumerPollOptions()
        ),
    ) -> PollerV2PollOut:
        """Poll messages from a sink."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return PollerV2PollOut.model_validate(response.json())

    async def consumer_commit(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        poller_v2_commit_in: PollerV2CommitIn,
        options: MessagePollerv2ConsumerCommitOptions = (
            MessagePollerv2ConsumerCommitOptions()
        ),
    ) -> None:
        """Ack a message offset for a sink's consumer."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}/commit",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=poller_v2_commit_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )


class MessagePollerv2(ApiBase):
    def consumer_poll(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        options: MessagePollerv2ConsumerPollOptions = (
            MessagePollerv2ConsumerPollOptions()
        ),
    ) -> PollerV2PollOut:
        """Poll messages from a sink."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return PollerV2PollOut.model_validate(response.json())

    def consumer_commit(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        poller_v2_commit_in: PollerV2CommitIn,
        options: MessagePollerv2ConsumerCommitOptions = (
            MessagePollerv2ConsumerCommitOptions()
        ),
    ) -> None:
        """Ack a message offset for a sink's consumer."""
        self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/polling-endpoint/{sink_id}/consumer/{consumer_id}/commit",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=poller_v2_commit_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
