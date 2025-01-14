import typing as t
from dataclasses import dataclass
from datetime import datetime


from ..internal.openapi_client.api.message import (
    v1_message_create,
    v1_message_expunge_content,
    v1_message_get,
    v1_message_list,
)
from ..internal.openapi_client.models.list_response_message_out import (
    ListResponseMessageOut,
)
from ..internal.openapi_client.models.message_in import MessageIn
from ..internal.openapi_client.models.message_out import MessageOut

from .common import ensure_tz, ListOptions, PostOptions, ApiBase


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
