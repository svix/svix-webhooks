import typing as t
from datetime import datetime
from dataclasses import dataclass

from deprecated import deprecated

from .common import ApiBase, BaseOptions, serialize_params
from ..internal.openapi_client import models


from ..internal.openapi_client.api.message_attempt import (
    v1_message_attempt_list_by_endpoint,
    v1_message_attempt_list_by_msg,
    v1_message_attempt_list_attempted_messages,
    v1_message_attempt_get,
    v1_message_attempt_expunge_content,
    v1_message_attempt_list_attempted_destinations,
    v1_message_attempt_resend,
)

from ..internal.openapi_client.models.list_response_message_attempt_out import (
    ListResponseMessageAttemptOut,
)
from ..internal.openapi_client.models.list_response_endpoint_message_out import (
    ListResponseEndpointMessageOut,
)
from ..internal.openapi_client.models.message_attempt_out import MessageAttemptOut
from ..internal.openapi_client.models.list_response_message_endpoint_out import (
    ListResponseMessageEndpointOut,
)
from ..internal.openapi_client.models.list_response_message_attempt_endpoint_out import (
    ListResponseMessageAttemptEndpointOut,
)


@dataclass
class MessageAttemptListByEndpointOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status: t.Optional[models.MessageStatus] = None
    # Filter response based on the HTTP status code
    status_code_class: t.Optional[models.StatusCodeClass] = None
    # Filter response based on the channel
    channel: t.Optional[str] = None
    # Filter response based on the tag
    tag: t.Optional[str] = None
    # Only include items created before a certain date
    before: t.Optional[datetime] = None
    # Only include items created after a certain date
    after: t.Optional[datetime] = None
    # When `true` attempt content is included in the response
    with_content: t.Optional[bool] = None
    # When `true`, the message information is included in the response
    with_msg: t.Optional[bool] = None
    # Filter response based on the event type
    event_types: t.Optional[t.Set[str]] = None


@dataclass
class MessageAttemptListByMsgOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status: t.Optional[models.MessageStatus] = None
    # Filter response based on the HTTP status code
    status_code_class: t.Optional[models.StatusCodeClass] = None
    # Filter response based on the channel
    channel: t.Optional[str] = None
    # Filter response based on the tag
    tag: t.Optional[str] = None
    # Filter the attempts based on the attempted endpoint
    endpoint_id: t.Optional[str] = None
    # Only include items created before a certain date
    before: t.Optional[datetime] = None
    # Only include items created after a certain date
    after: t.Optional[datetime] = None
    # When `true` attempt content is included in the response
    with_content: t.Optional[bool] = None
    # Filter response based on the event type
    event_types: t.Optional[t.Set[str]] = None


@dataclass
class MessageAttemptListAttemptedMessagesOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # Filter response based on the channel
    channel: t.Optional[str] = None
    # Filter response based on the message tags
    tag: t.Optional[str] = None
    # Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status: t.Optional[models.MessageStatus] = None
    # Only include items created before a certain date
    before: t.Optional[datetime] = None
    # Only include items created after a certain date
    after: t.Optional[datetime] = None
    # When `true` message payloads are included in the response
    with_content: t.Optional[bool] = None
    # Filter response based on the event type
    event_types: t.Optional[t.Set[str]] = None


@dataclass
class MessageAttemptListByMsgDeprecatedOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # Filter the attempts based on the attempted endpoint
    endpoint_id: t.Optional[str] = None
    # Filter response based on the channel
    channel: t.Optional[str] = None
    # Filter response based on the tag
    tag: t.Optional[str] = None
    # Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status: t.Optional[models.MessageStatus] = None
    # Only include items created before a certain date
    before: t.Optional[datetime] = None
    # Only include items created after a certain date
    after: t.Optional[datetime] = None
    # Filter response based on the HTTP status code
    status_code_class: t.Optional[models.StatusCodeClass] = None
    # Filter response based on the event type
    event_types: t.Optional[t.Set[str]] = None

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "limit": self.limit,
                "iterator": self.iterator,
                "endpoint_id": self.endpoint_id,
                "channel": self.channel,
                "tag": self.tag,
                "status": self.status,
                "before": self.before,
                "after": self.after,
                "status_code_class": self.status_code_class,
                "event_types": self.event_types,
            }
        )


@dataclass
class MessageAttemptListAttemptedDestinationsOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None


@dataclass
class MessageAttemptListByEndpointDeprecatedOptions(BaseOptions):
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # Filter response based on the channel
    channel: t.Optional[str] = None
    # Filter response based on the tag
    tag: t.Optional[str] = None
    # Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3)
    status: t.Optional[models.MessageStatus] = None
    # Only include items created before a certain date
    before: t.Optional[datetime] = None
    # Only include items created after a certain date
    after: t.Optional[datetime] = None
    # Filter response based on the event type
    event_types: t.Optional[t.Set[str]] = None

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
                "event_types": self.event_types,
            }
        )


@dataclass
class MessageAttemptResendOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


@dataclass
class MessageListAttemptsForEndpointOptions(BaseOptions):
    status: t.Optional[models.MessageStatus] = None
    event_types: t.Optional[t.List[str]] = None
    before: t.Optional[datetime] = None
    after: t.Optional[datetime] = None
    channel: t.Optional[str] = None
    status_code_class: t.Optional[models.StatusCodeClass] = None


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
        return await v1_message_attempt_list_by_endpoint.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

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
        return await v1_message_attempt_list_by_msg.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

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
        return await v1_message_attempt_list_attempted_messages.request_asyncio(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    @deprecated
    async def list_by_msg_deprecated(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListByMsgDeprecatedOptions = MessageAttemptListByMsgDeprecatedOptions(),
    ) -> ListResponseMessageAttemptOut:
        """Deprecated: Please use "List Attempts by Endpoint" and "List Attempts by Msg" instead.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.

        `msg_id`: Use a message id or a message `eventId`"""
        # ruff: noqa: F841
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/attempt",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptOut.from_dict(response.json())

    async def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        """`msg_id`: Use a message id or a message `eventId`"""
        return await v1_message_attempt_get.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, attempt_id=attempt_id
        )

    async def expunge_content(self, app_id: str, msg_id: str, attempt_id: str) -> None:
        """Deletes the given attempt's response body.

        Useful when an endpoint accidentally returned sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        return await v1_message_attempt_expunge_content.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, attempt_id=attempt_id
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
        return await v1_message_attempt_list_attempted_destinations.request_asyncio(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    @deprecated
    async def list_by_endpoint_deprecated(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptListByEndpointDeprecatedOptions = MessageAttemptListByEndpointDeprecatedOptions(),
    ) -> ListResponseMessageAttemptEndpointOut:
        """DEPRECATED: please use list_attempts with endpoint_id as a query parameter instead.

        List the message attempts for a particular endpoint.

        Returning the endpoint.

        The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        # ruff: noqa: F841
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/attempt",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptEndpointOut.from_dict(response.json())

    async def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptResendOptions = MessageAttemptResendOptions(),
    ) -> None:
        """Resend a message to the specified endpoint."""
        return await v1_message_attempt_resend.request_asyncio(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
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
        return v1_message_attempt_list_by_endpoint.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

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
        return v1_message_attempt_list_by_msg.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

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
        return v1_message_attempt_list_attempted_messages.request_sync(
            client=self._client,
            app_id=app_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )

    @deprecated
    def list_by_msg_deprecated(
        self,
        app_id: str,
        msg_id: str,
        options: MessageAttemptListByMsgDeprecatedOptions = MessageAttemptListByMsgDeprecatedOptions(),
    ) -> ListResponseMessageAttemptOut:
        """Deprecated: Please use "List Attempts by Endpoint" and "List Attempts by Msg" instead.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.

        `msg_id`: Use a message id or a message `eventId`"""
        # ruff: noqa: F841
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/attempt",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptOut.from_dict(response.json())

    def get(self, app_id: str, msg_id: str, attempt_id: str) -> MessageAttemptOut:
        """`msg_id`: Use a message id or a message `eventId`"""
        return v1_message_attempt_get.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id, attempt_id=attempt_id
        )

    def expunge_content(self, app_id: str, msg_id: str, attempt_id: str) -> None:
        """Deletes the given attempt's response body.

        Useful when an endpoint accidentally returned sensitive content.
        The message can't be replayed or resent once its payload has been deleted or expired."""
        return v1_message_attempt_expunge_content.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id, attempt_id=attempt_id
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
        return v1_message_attempt_list_attempted_destinations.request_sync(
            client=self._client, app_id=app_id, msg_id=msg_id, **options.to_dict()
        )

    @deprecated
    def list_by_endpoint_deprecated(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptListByEndpointDeprecatedOptions = MessageAttemptListByEndpointDeprecatedOptions(),
    ) -> ListResponseMessageAttemptEndpointOut:
        """DEPRECATED: please use list_attempts with endpoint_id as a query parameter instead.

        List the message attempts for a particular endpoint.

        Returning the endpoint.

        The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.

        Note that by default this endpoint is limited to retrieving 90 days' worth of data
        relative to now or, if an iterator is provided, 90 days before/after the time indicated
        by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
        set the `before` or `after` parameter as appropriate.
        """
        # ruff: noqa: F841
        response = self._request_sync(
            method="get",
            path="/api/v1/app/{app_id}/msg/{msg_id}/endpoint/{endpoint_id}/attempt",
            path_params={
                "app_id": app_id,
                "msg_id": msg_id,
                "endpoint_id": endpoint_id,
            },
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseMessageAttemptEndpointOut.from_dict(response.json())

    def resend(
        self,
        app_id: str,
        msg_id: str,
        endpoint_id: str,
        options: MessageAttemptResendOptions = MessageAttemptResendOptions(),
    ) -> None:
        """Resend a message to the specified endpoint."""
        return v1_message_attempt_resend.request_sync(
            client=self._client,
            app_id=app_id,
            msg_id=msg_id,
            endpoint_id=endpoint_id,
            **options.to_dict(),
        )
