import typing as t
from datetime import datetime
from dataclasses import dataclass

from .common import ApiBase, BaseOptions


from ..internal.openapi_client.api.message import (
    v1_message_list,
    v1_message_create,
    v1_message_get,
    v1_message_expunge_content,
)

from ..internal.openapi_client.models.list_response_message_out import (
    ListResponseMessageOut,
)
from ..internal.openapi_client.models.message_in import MessageIn
from ..internal.openapi_client.models.message_out import MessageOut


@dataclass
class MessageListOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # Filter response based on the channel.
    channel: t.Optional[str] = None
    # Only include items created before a certain date.
    before: t.Optional[datetime] = None
    # Only include items created after a certain date.
    after: t.Optional[datetime] = None
    # When `true` message payloads are included in the response.
    with_content: t.Optional[bool] = None
    # Filter messages matching the provided tag.
    tag: t.Optional[str] = None
    # Filter response based on the event type
    event_types: t.Optional[t.Set[str]] = None


@dataclass
class MessageCreateOptions(BaseOptions):
    # When `true`, message payloads are included in the response.
    with_content: t.Optional[bool] = False

    idempotency_key: t.Optional[str] = None


@dataclass
class MessageGetOptions(BaseOptions):
    # When `true` message payloads are included in the response.
    with_content: t.Optional[bool] = None


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
    async def list(
        self, app_id: str, options: MessageListOptions = MessageListOptions()
    ) -> ListResponseMessageOut:
        """List all of the application's messages.

        The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
        within a certain window.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        return await v1_message_list.request_asyncio(
            client=self._client, app_id=app_id, **options.to_dict()
        )

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

        The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb."""
        ret = await v1_message_create.request_asyncio(
            client=self._client,
            app_id=app_id,
            json_body=message_in,
            **options.to_dict(),
        )
        ret.payload = message_in.payload
        return ret

    async def get(
        self, app_id: str, msg_id: str, options: MessageGetOptions = MessageGetOptions()
    ) -> MessageOut:
        """Get a message by its ID or eventID."""
        return await v1_message_get.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    async def expunge_content(self, app_id: str, msg_id: str) -> None:
        """Delete the given message's payload.

        Useful in cases when a message was accidentally sent with sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        return await v1_message_expunge_content.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id
        )


class Message(ApiBase):
    def list(
        self, app_id: str, options: MessageListOptions = MessageListOptions()
    ) -> ListResponseMessageOut:
        """List all of the application's messages.

        The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
        within a certain window.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        return v1_message_list.request_sync(
            client=self._client, app_id=app_id, **options.to_dict()
        )

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

        The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb."""
        ret = v1_message_create.request_sync(
            client=self._client,
            app_id=app_id,
            json_body=message_in,
            **options.to_dict(),
        )
        ret.payload = message_in.payload
        return ret

    def get(
        self, app_id: str, msg_id: str, options: MessageGetOptions = MessageGetOptions()
    ) -> MessageOut:
        """Get a message by its ID or eventID."""
        return v1_message_get.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    def expunge_content(self, app_id: str, msg_id: str) -> None:
        """Delete the given message's payload.

        Useful in cases when a message was accidentally sent with sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        return v1_message_expunge_content.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id
        )
