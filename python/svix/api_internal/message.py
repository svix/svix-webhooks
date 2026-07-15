# This file is @generated
from .common import ApiBaseAsync, ApiBaseSync
from .message_pollerv2 import (
    MessagePollerv2,
    MessagePollerv2Async,
)


class MessageAsync(ApiBaseAsync):
    @property
    def pollerv2(self) -> MessagePollerv2Async:
        return MessagePollerv2Async(self._client, self._httpx_client)


class Message(ApiBaseSync):
    @property
    def pollerv2(self) -> MessagePollerv2:
        return MessagePollerv2(self._client, self._httpx_client)
