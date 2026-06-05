# This file is @generated
import typing as t
from dataclasses import dataclass
from datetime import datetime

from ..models import (
    PollingEndpointConsumerSeekIn,
    PollingEndpointConsumerSeekOut,
    PollingEndpointOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class MessagePollerPollOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    event_type: t.Optional[str] = None
    """Filters messages sent with this event type (optional)."""
    channel: t.Optional[str] = None
    """Filters messages sent with this channel (optional)."""
    after: t.Optional[datetime] = None

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "event_type": self.event_type,
                "channel": self.channel,
                "after": self.after,
            }
        )


@dataclass
class MessagePollerConsumerPollOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
            }
        )


@dataclass
class MessagePollerConsumerSeekOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class MessagePollerAsync(ApiBase):
    async def poll(
        self,
        app_id: str,
        sink_id: str,
        options: MessagePollerPollOptions = MessagePollerPollOptions(),
    ) -> PollingEndpointOut:
        """Reads the stream of created messages for an application, filtered on the Sink's event types and Channels."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/poller/{sink_id}",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return PollingEndpointOut.model_validate(response.json())

    async def consumer_poll(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        options: MessagePollerConsumerPollOptions = MessagePollerConsumerPollOptions(),
    ) -> PollingEndpointOut:
        """Reads the stream of created messages for an application, filtered on the Sink's event types and
        Channels, using server-managed iterator tracking."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return PollingEndpointOut.model_validate(response.json())

    async def consumer_seek(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        polling_endpoint_consumer_seek_in: PollingEndpointConsumerSeekIn,
        options: MessagePollerConsumerSeekOptions = MessagePollerConsumerSeekOptions(),
    ) -> PollingEndpointConsumerSeekOut:
        """Sets the starting offset for the consumer of a polling endpoint."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=polling_endpoint_consumer_seek_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return PollingEndpointConsumerSeekOut.model_validate(response.json())


class MessagePoller(ApiBase):
    def poll(
        self,
        app_id: str,
        sink_id: str,
        options: MessagePollerPollOptions = MessagePollerPollOptions(),
    ) -> PollingEndpointOut:
        """Reads the stream of created messages for an application, filtered on the Sink's event types and Channels."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/poller/{sink_id}",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return PollingEndpointOut.model_validate(response.json())

    def consumer_poll(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        options: MessagePollerConsumerPollOptions = MessagePollerConsumerPollOptions(),
    ) -> PollingEndpointOut:
        """Reads the stream of created messages for an application, filtered on the Sink's event types and
        Channels, using server-managed iterator tracking."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return PollingEndpointOut.model_validate(response.json())

    def consumer_seek(
        self,
        app_id: str,
        sink_id: str,
        consumer_id: str,
        polling_endpoint_consumer_seek_in: PollingEndpointConsumerSeekIn,
        options: MessagePollerConsumerSeekOptions = MessagePollerConsumerSeekOptions(),
    ) -> PollingEndpointConsumerSeekOut:
        """Sets the starting offset for the consumer of a polling endpoint."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek",
            path_params={
                "app_id": app_id,
                "sink_id": sink_id,
                "consumer_id": consumer_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=polling_endpoint_consumer_seek_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return PollingEndpointConsumerSeekOut.model_validate(response.json())
