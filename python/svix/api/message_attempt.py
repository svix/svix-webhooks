import typing as t
from dataclasses import dataclass
from datetime import datetime

from deprecated import deprecated

from .common import ensure_tz, ListOptions, PostOptions, ApiBase


from ..internal.openapi_client.api.message_attempt import (
    v1_message_attempt_expunge_content,
    v1_message_attempt_get,
    v1_message_attempt_list_attempted_destinations,
    v1_message_attempt_list_attempted_messages,
    v1_message_attempt_list_by_endpoint,
    v1_message_attempt_list_by_endpoint_deprecated,
    v1_message_attempt_list_by_msg,
    v1_message_attempt_resend,
)
from ..internal.openapi_client.models.list_response_endpoint_message_out import (
    ListResponseEndpointMessageOut,
)
from ..internal.openapi_client.models.list_response_message_attempt_endpoint_out import (
    ListResponseMessageAttemptEndpointOut,
)
from ..internal.openapi_client.models.list_response_message_attempt_out import (
    ListResponseMessageAttemptOut,
)
from ..internal.openapi_client.models.list_response_message_endpoint_out import (
    ListResponseMessageEndpointOut,
)
from ..internal.openapi_client.models.message_attempt_out import MessageAttemptOut
from ..internal.openapi_client.models.message_status import MessageStatus
from ..internal.openapi_client.models.status_code_class import StatusCodeClass


@dataclass
class MessageAttemptListOptions(ListOptions):
    status: t.Optional[MessageStatus] = None
    event_types: t.Optional[t.List[str]] = None
    before: t.Optional[datetime] = None
    after: t.Optional[datetime] = None
    channel: t.Optional[str] = None
    status_code_class: t.Optional[StatusCodeClass] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        d = super().to_dict()
        if self.before is not None:
            d["before"] = ensure_tz(self.before)
        if self.after is not None:
            d["after"] = ensure_tz(self.after)
        return d


class MessageAttemptAsync(ApiBase):
    async def list_by_msg(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return await v1_message_attempt_list_by_msg.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    async def list_by_endpoint(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return await v1_message_attempt_list_by_endpoint.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        return await v1_message_attempt_get.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )

    async def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: PostOptions = PostOptions(),
    ) -> None:
        return await v1_message_attempt_resend.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def list_attempted_messages(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseEndpointMessageOut:
        return await v1_message_attempt_list_attempted_messages.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def list_attempted_destinations(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageEndpointOut:
        return await v1_message_attempt_list_attempted_destinations.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            **options.to_dict(),
        )

    @deprecated(
        reason="use list_by_msg instead, passing the endpoint id through options"
    )
    async def list_attempts_for_endpoint(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptEndpointOut:
        return await v1_message_attempt_list_by_endpoint_deprecated.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    async def expunge_content(
        self,
        app_id: str,
        msg_id: str,
        attempt_id: str,
    ) -> None:
        return await v1_message_attempt_expunge_content.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )


class MessageAttempt(ApiBase):
    @deprecated(reason="use list_by_msg or list_by_endpoint instead")
    def list(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return self.list_by_msg(app_id=app_id, msg_id=msg_id, options=options)

    def list_by_msg(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return v1_message_attempt_list_by_msg.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    def list_by_endpoint(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptOut:
        return v1_message_attempt_list_by_endpoint.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        return v1_message_attempt_get.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )

    def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: PostOptions = PostOptions(),
    ) -> None:
        return v1_message_attempt_resend.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def list_attempted_messages(
        self,
        app_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseEndpointMessageOut:
        return v1_message_attempt_list_attempted_messages.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def list_attempted_destinations(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageEndpointOut:
        return v1_message_attempt_list_attempted_destinations.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            **options.to_dict(),
        )

    @deprecated(
        reason="use list_by_msg instead, passing the endpoint id through options"
    )
    def list_attempts_for_endpoint(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptListOptions = MessageAttemptListOptions(),
    ) -> ListResponseMessageAttemptEndpointOut:
        return v1_message_attempt_list_by_endpoint_deprecated.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    def expunge_content(
        self,
        app_id: str,
        msg_id: str,
        attempt_id: str,
    ) -> None:
        return v1_message_attempt_expunge_content.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            attempt_id=attempt_id,
        )
