# This file is @generated
import typing as t
from dataclasses import dataclass
from datetime import datetime

from .. import models
from ..models import (
    ListResponseEndpointMessageOut,
    ListResponseMessageAttemptOut,
    ListResponseMessageEndpointOut,
    MessageAttemptOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class MessageAttemptListByEndpointOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    status: t.Optional[models.MessageStatus] = None
    """Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)"""
    status_code_class: t.Optional[models.StatusCodeClass] = None
    """Filter response based on the HTTP status code"""
    channel: t.Optional[str] = None
    """Filter response based on the channel"""
    tag: t.Optional[str] = None
    """Filter response based on the tag"""
    before: t.Optional[datetime] = None
    """Only include items created before a certain date"""
    after: t.Optional[datetime] = None
    """Only include items created after a certain date"""
    with_content: t.Optional[bool] = None
    """When `true` attempt content is included in the response"""
    with_msg: t.Optional[bool] = None
    """When `true`, the message information is included in the response"""
    event_types: t.Optional[t.List[str]] = None
    """Filter response based on the event type"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "status": self.status,
                "status_code_class": self.status_code_class,
                "channel": self.channel,
                "tag": self.tag,
                "before": self.before,
                "after": self.after,
                "with_content": self.with_content,
                "with_msg": self.with_msg,
                "event_types": self.event_types,
            }
        )


@dataclass
class MessageAttemptListByMsgOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    status: t.Optional[models.MessageStatus] = None
    """Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)"""
    status_code_class: t.Optional[models.StatusCodeClass] = None
    """Filter response based on the HTTP status code"""
    channel: t.Optional[str] = None
    """Filter response based on the channel"""
    tag: t.Optional[str] = None
    """Filter response based on the tag"""
    endpoint_id: t.Optional[str] = None
    """Filter the attempts based on the attempted endpoint"""
    before: t.Optional[datetime] = None
    """Only include items created before a certain date"""
    after: t.Optional[datetime] = None
    """Only include items created after a certain date"""
    with_content: t.Optional[bool] = None
    """When `true` attempt content is included in the response"""
    event_types: t.Optional[t.List[str]] = None
    """Filter response based on the event type"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "status": self.status,
                "status_code_class": self.status_code_class,
                "channel": self.channel,
                "tag": self.tag,
                "endpoint_id": self.endpoint_id,
                "before": self.before,
                "after": self.after,
                "with_content": self.with_content,
                "event_types": self.event_types,
            }
        )


@dataclass
class MessageAttemptListAttemptedMessagesOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    channel: t.Optional[str] = None
    """Filter response based on the channel"""
    tag: t.Optional[str] = None
    """Filter response based on the message tags"""
    status: t.Optional[models.MessageStatus] = None
    """Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)"""
    before: t.Optional[datetime] = None
    """Only include items created before a certain date"""
    after: t.Optional[datetime] = None
    """Only include items created after a certain date"""
    with_content: t.Optional[bool] = None
    """When `true` message payloads are included in the response"""
    event_types: t.Optional[t.List[str]] = None
    """Filter response based on the event type"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "channel": self.channel,
                "tag": self.tag,
                "status": self.status,
                "before": self.before,
                "after": self.after,
                "with_content": self.with_content,
                "event_types": self.event_types,
            }
        )


@dataclass
class MessageAttemptListAttemptedDestinationsOptions(BaseOptions):
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
class MessageAttemptResendOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class MessageAttemptAsync(ApiBase):
    async def list_by_endpoint(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListByEndpointOptions = MessageAttemptListByEndpointOptions(),
    ) -> ListResponseMessageAttemptOut:
        """List attempts by endpoint id

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptOut.model_validate(response.json())

    async def list_by_msg(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListByMsgOptions = MessageAttemptListByMsgOptions(),
    ) -> ListResponseMessageAttemptOut:
        """List attempts by message ID.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/attempt/msg/{msg_id}",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptOut.model_validate(response.json())

    async def list_attempted_messages(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListAttemptedMessagesOptions = MessageAttemptListAttemptedMessagesOptions(),
    ) -> ListResponseEndpointMessageOut:
        """List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.

        The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseEndpointMessageOut.model_validate(response.json())

    async def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        """`msg_id`: Use a message id or a message `eventId`"""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "attempt_id": attempt_id,
            },
        )
        return MessageAttemptOut.model_validate(response.json())

    async def expunge_content(self, app_id: str, msg_id: str, attempt_id: str) -> None:
        """Deletes the given attempt's response body.

        Useful when an endpoint accidentally returned sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "attempt_id": attempt_id,
            },
        )

    async def list_attempted_destinations(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListAttemptedDestinationsOptions = MessageAttemptListAttemptedDestinationsOptions(),
    ) -> ListResponseMessageEndpointOut:
        """List endpoints attempted by a given message.

        Additionally includes metadata about the latest message attempt.
        By default, endpoints are listed in ascending order by ID."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/endpoint",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageEndpointOut.model_validate(response.json())

    async def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptResendOptions = MessageAttemptResendOptions(),
    ) -> None:
        """Resend a message to the specified endpoint."""
        await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )


class MessageAttempt(ApiBase):
    def list_by_endpoint(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListByEndpointOptions = MessageAttemptListByEndpointOptions(),
    ) -> ListResponseMessageAttemptOut:
        """List attempts by endpoint id

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/attempt/endpoint/{endpoint_id}",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptOut.model_validate(response.json())

    def list_by_msg(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListByMsgOptions = MessageAttemptListByMsgOptions(),
    ) -> ListResponseMessageAttemptOut:
        """List attempts by message ID.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/attempt/msg/{msg_id}",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptOut.model_validate(response.json())

    def list_attempted_messages(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListAttemptedMessagesOptions = MessageAttemptListAttemptedMessagesOptions(),
    ) -> ListResponseEndpointMessageOut:
        """List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.

        The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg",
            path_params={
                "app_id": app_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseEndpointMessageOut.model_validate(response.json())

    def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        """`msg_id`: Use a message id or a message `eventId`"""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "attempt_id": attempt_id,
            },
        )
        return MessageAttemptOut.model_validate(response.json())

    def expunge_content(self, app_id: str, msg_id: str, attempt_id: str) -> None:
        """Deletes the given attempt's response body.

        Useful when an endpoint accidentally returned sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        self._request_sync(
            method="delete",
            path="/api/v1/app/{app_id}/msg/{msg_id}/attempt/{attempt_id}/content",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "attempt_id": attempt_id,
            },
        )

    def list_attempted_destinations(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListAttemptedDestinationsOptions = MessageAttemptListAttemptedDestinationsOptions(),
    ) -> ListResponseMessageEndpointOut:
        """List endpoints attempted by a given message.

        Additionally includes metadata about the latest message attempt.
        By default, endpoints are listed in ascending order by ID."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/endpoint",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageEndpointOut.model_validate(response.json())

    def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptResendOptions = MessageAttemptResendOptions(),
    ) -> None:
        """Resend a message to the specified endpoint."""
        self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/resend",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
