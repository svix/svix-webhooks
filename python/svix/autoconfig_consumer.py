import typing as t

from .api.client import AuthenticatedClient
from .api.svix import Svix, SvixOptions
from .api_internal.endpoint_auto_config import (
    EndpointAutoConfig,
    EndpointAutoConfigAsync,
)
from .api_internal.message_pollerv2 import (
    MessagePollerv2,
    MessagePollerv2Async,
    MessagePollerv2ConsumerCommitOptions,
    MessagePollerv2ConsumerPollOptions,
)
from .autoconfig import decode_autoconfig_token_v1
from .models import (
    AutoConfigSinkType,
    EndpointOut,
    PollerV2CommitIn,
    PollerV2PollOut,
    SinkInCommon,
    SubscribeIn,
)


class AutoConfigConsumer:
    _app_id: str
    _sink_id: str
    _sink_in: SinkInCommon
    _client: AuthenticatedClient

    def __init__(self, token: str, sink_in: SinkInCommon) -> None:
        content = decode_autoconfig_token_v1(token)

        svix = Svix(
            content.token_plaintext,
            SvixOptions(server_url=content.server_url),
        )

        self._app_id = content.app_id
        self._sink_id = content.endpoint_id
        self._sink_in = sink_in
        self._client = svix._client

    def _subscribe_in(self) -> SubscribeIn:
        return SubscribeIn(
            sink=AutoConfigSinkType(type="poller", config=self._sink_in),
        )

    def subscribe(self) -> EndpointOut:
        return EndpointAutoConfig(self._client).update(
            self._app_id,
            self._sink_id,
            self._subscribe_in(),
        )

    async def subscribe_async(self) -> EndpointOut:
        return await EndpointAutoConfigAsync(self._client).update(
            self._app_id,
            self._sink_id,
            self._subscribe_in(),
        )

    def receive(
        self,
        consumer_id: str,
        options: MessagePollerv2ConsumerPollOptions = MessagePollerv2ConsumerPollOptions(),
    ) -> PollerV2PollOut:
        return MessagePollerv2(self._client).consumer_poll(
            self._app_id,
            self._sink_id,
            consumer_id,
            options,
        )

    async def receive_async(
        self,
        consumer_id: str,
        options: MessagePollerv2ConsumerPollOptions = MessagePollerv2ConsumerPollOptions(),
    ) -> PollerV2PollOut:
        return await MessagePollerv2Async(self._client).consumer_poll(
            self._app_id,
            self._sink_id,
            consumer_id,
            options,
        )

    def commit(
        self,
        consumer_id: str,
        offset: int,
        options: MessagePollerv2ConsumerCommitOptions = MessagePollerv2ConsumerCommitOptions(),
    ) -> None:
        MessagePollerv2(self._client).consumer_commit(
            self._app_id,
            self._sink_id,
            consumer_id,
            PollerV2CommitIn(offset=offset),
            options,
        )

    async def commit_async(
        self,
        consumer_id: str,
        offset: int,
        options: MessagePollerv2ConsumerCommitOptions = MessagePollerv2ConsumerCommitOptions(),
    ) -> None:
        await MessagePollerv2Async(self._client).consumer_commit(
            self._app_id,
            self._sink_id,
            consumer_id,
            PollerV2CommitIn(offset=offset),
            options,
        )
