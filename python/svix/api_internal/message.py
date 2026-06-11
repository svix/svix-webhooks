# This file is @generated
from .common import ApiBase
from .message_pollerv2 import (
    MessagePollerv2,
    MessagePollerv2Async,
)


class MessageAsync(ApiBase):
    @property
    def pollerv2(self) -> MessagePollerv2Async:
        return MessagePollerv2Async(self._client)


class Message(ApiBase):
    @property
    def pollerv2(self) -> MessagePollerv2:
        return MessagePollerv2(self._client)
