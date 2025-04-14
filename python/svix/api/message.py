# This file is @generated
import typing as t
from dataclasses import dataclass
from datetime import datetime

from ..models import (
    ExpungeAllContentsOut,
    ListResponseMessageOut,
    MessageIn,
    MessageOut,
)
from .common import ApiBase, BaseOptions, serialize_params
from .message_poller import (
    MessagePoller,
    MessagePollerAsync,
)


@dataclass
class MessageListOptions(BaseOptions):
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    channel: t.Optional[str] = None
    """Filter response based on the channel."""
    before: t.Optional[datetime] = None
    """Only include items created before a certain date."""
    after: t.Optional[datetime] = None
    """Only include items created after a certain date."""
    with_content: t.Optional[bool] = None
    """When `true` message payloads are included in the response."""
    tag: t.Optional[str] = None
    """Filter messages matching the provided tag."""
    event_types: t.Optional[t.List[str]] = None
    """Filter response based on the event type"""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "channel": self.channel,
                "before": self.before,
                "after": self.after,
                "with_content": self.with_content,
                "tag": self.tag,
                "event_types": self.event_types,
            }
        )


@dataclass
class MessageCreateOptions(BaseOptions):
    with_content: t.Optional[bool] = None
    """When `true`, message payloads are included in the response."""
    idempotency_key: t.Optional[str] = None

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "with_content": self.with_content,
            }
        )

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class MessageExpungeAllContentsOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


@dataclass
class MessageGetOptions(BaseOptions):
    with_content: t.Optional[bool] = None
    """When `true` message payloads are included in the response."""

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "with_content": self.with_content,
            }
        )


def message_in_raw(
    event_type: str, payload: str, content_type: t.Optional[str] = None
) -> MessageIn:
    """
    Creates a `MessageIn` with a raw string payload.

    The payload is not normalized on the server. Normally, payloads are required
    to be JSON, and Svix will minify the payload before sending the webhook
    (for example, by removing extraneous whitespace or unnecessarily escaped
    characters in strings). With this function, the payload will be sent
    "as is", without any minification or other processing.

    Args:
        event_type (str): The event type's name Example: `user.signup`.
        payload (str): Serialized message payload.
        content_type (str?): The value to use for the Content-Type header of the
            webhook sent by Svix, overwriting the default of `application/json`
            if specified.
    """
    transformations_params: t.Dict[str, t.Any] = {
        "rawPayload": payload,
    }
    if content_type is not None:
        transformations_params["headers"] = {"content-type": content_type}

    return MessageIn(
        event_type=event_type,
        payload={},
        transformations_params=transformations_params,
    )


class MessageAsync(ApiBase):
    @property
    def poller(self) -> MessagePollerAsync:
        return MessagePollerAsync(self._client)

    async def list(
        self, app_id: str, options: MessageListOptions = MessageListOptions()
    ) -> ListResponseMessageOut:
        """List all of the application's messages.

        The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
        within a certain window.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/msg",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageOut.model_validate(response.json())

    async def create(
        self,
        app_id: str,
        message_in: MessageIn,
        options: MessageCreateOptions = MessageCreateOptions(),
    ) -> MessageOut:
        """Creates a new message and dispatches it to all of the application's endpoints.

        The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
        If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.

        The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
        Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.

        The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/msg",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=message_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return MessageOut.model_validate(response.json())

    async def expunge_all_contents(
        self,
        app_id: str,
        options: MessageExpungeAllContentsOptions = MessageExpungeAllContentsOptions(),
    ) -> ExpungeAllContentsOut:
        """Delete all message payloads for the application.

        This operation is only available in the <a href="https://svix.com/pricing" target="_blank">Enterprise</a> plan."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/app/{app_id}/msg/expunge-all-contents",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ExpungeAllContentsOut.model_validate(response.json())

    async def get(
        self, app_id: str, msg_id: str, options: MessageGetOptions = MessageGetOptions()
    ) -> MessageOut:
        """Get a message by its ID or eventID."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return MessageOut.model_validate(response.json())

    async def expunge_content(self, app_id: str, msg_id: str) -> None:
        """Delete the given message's payload.

        Useful in cases when a message was accidentally sent with sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        await self._request_asyncio(
            method="delete",
            path="/api/v1/app/{app_id}/msg/{msg_id}/content",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
        )


class Message(ApiBase):
    @property
    def poller(self) -> MessagePoller:
        return MessagePoller(self._client)

    def list(
        self, app_id: str, options: MessageListOptions = MessageListOptions()
    ) -> ListResponseMessageOut:
        """List all of the application's messages.

        The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
        within a certain window.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/msg",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageOut.model_validate(response.json())

    def create(
        self,
        app_id: str,
        message_in: MessageIn,
        options: MessageCreateOptions = MessageCreateOptions(),
    ) -> MessageOut:
        """Creates a new message and dispatches it to all of the application's endpoints.

        The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
        If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.

        The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
        Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.

        The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to 1MiB, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/msg",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=message_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return MessageOut.model_validate(response.json())

    def expunge_all_contents(
        self,
        app_id: str,
        options: MessageExpungeAllContentsOptions = MessageExpungeAllContentsOptions(),
    ) -> ExpungeAllContentsOut:
        """Delete all message payloads for the application.

        This operation is only available in the <a href="https://svix.com/pricing" target="_blank">Enterprise</a> plan."""
        response = self._request_sync(
            method="post",
            path="/api/v1/app/{app_id}/msg/expunge-all-contents",
            path_params={
                "app_id": app_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ExpungeAllContentsOut.model_validate(response.json())

    def get(
        self, app_id: str, msg_id: str, options: MessageGetOptions = MessageGetOptions()
    ) -> MessageOut:
        """Get a message by its ID or eventID."""
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return MessageOut.model_validate(response.json())

    def expunge_content(self, app_id: str, msg_id: str) -> None:
        """Delete the given message's payload.

        Useful in cases when a message was accidentally sent with sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        self._request_sync(
            method="delete",
            path="/api/v1/app/{app_id}/msg/{msg_id}/content",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
        )
