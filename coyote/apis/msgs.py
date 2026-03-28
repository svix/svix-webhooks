# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    MsgPublishIn,
    MsgPublishOut,
)
from .msgs_namespace import (
    MsgsNamespace,
    MsgsNamespaceAsync,
)
from .msgs_queue import (
    MsgsQueue,
    MsgsQueueAsync,
)
from .msgs_stream import (
    MsgsStream,
    MsgsStreamAsync,
)
from .msgs_topic import (
    MsgsTopic,
    MsgsTopicAsync,
)

from ..models.msg_publish_in import _MsgPublishIn


class MsgsAsync(ApiBase):
    @property
    def namespace(self) -> MsgsNamespaceAsync:
        return MsgsNamespaceAsync(self._client)

    @property
    def queue(self) -> MsgsQueueAsync:
        return MsgsQueueAsync(self._client)

    @property
    def stream(self) -> MsgsStreamAsync:
        return MsgsStreamAsync(self._client)

    @property
    def topic(self) -> MsgsTopicAsync:
        return MsgsTopicAsync(self._client)

    async def publish(
        self,
        topic: str,
        msg_publish_in: MsgPublishIn,
    ) -> MsgPublishOut:
        """Publishes messages to a topic within a namespace."""
        body = _MsgPublishIn(
            namespace=msg_publish_in.namespace,
            topic=topic,
            msgs=msg_publish_in.msgs,
        ).model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.msgs.publish",
            body=body,
            response_type=MsgPublishOut,
        )


class Msgs(ApiBase):
    @property
    def namespace(self) -> MsgsNamespace:
        return MsgsNamespace(self._client)

    @property
    def queue(self) -> MsgsQueue:
        return MsgsQueue(self._client)

    @property
    def stream(self) -> MsgsStream:
        return MsgsStream(self._client)

    @property
    def topic(self) -> MsgsTopic:
        return MsgsTopic(self._client)

    def publish(
        self,
        topic: str,
        msg_publish_in: MsgPublishIn,
    ) -> MsgPublishOut:
        """Publishes messages to a topic within a namespace."""
        body = _MsgPublishIn(
            namespace=msg_publish_in.namespace,
            topic=topic,
            msgs=msg_publish_in.msgs,
        ).model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.msgs.publish",
            body=body,
            response_type=MsgPublishOut,
        )
