import base64
import typing as t

import httpx
import pydantic

from .api.client import AuthenticatedClient
from .api.common import _make_httpx_async_client, _make_httpx_client
from .api.svix import Svix, SvixOptions
from .api_internal.destination_autoconfig import (
    DestinationAutoconfig,
    DestinationAutoconfigAsync,
)
from .api_internal.endpoint_auto_config_deprecated import (
    EndpointAutoConfigDeprecated,
    EndpointAutoConfigDeprecatedAsync,
)
from .api_internal.endpoint_autoconfig import (
    EndpointAutoconfig,
    EndpointAutoconfigAsync,
)
from .api_internal.message_pollerv2 import (
    MessagePollerv2,
    MessagePollerv2Async,
    MessagePollerv2ConsumerCommitOptions,
    MessagePollerv2ConsumerPollOptions,
)
from .models import (
    AutoConfigSinkType,
    DestinationIn,
    DestinationOut,
    EndpointIn,
    EndpointOut,
    PollerV2CommitIn,
    PollerV2PollOut,
    SinkInCommon,
    SinkStatus,
    SubscribeIn,
)
from .webhooks import Webhook

_AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"
_AUTOCONFIG_TOKEN_PREFIX_V2 = "auto_v2_"
_UNSUPPORTED_TOKEN_VERSION = (
    "Unsupported token version. You might need to update the Svix SDK to use this token"
)


class AutoConfigError(Exception):
    """Raised when an autoconfig token cannot be decoded or validated."""


class _AutoConfigTokenContentV1(pydantic.BaseModel):
    app_id: str = pydantic.Field(alias="aid")
    endpoint_id: str = pydantic.Field(alias="eid")
    server_url: str = pydantic.Field(alias="surl")
    endpoint_secret: str = pydantic.Field(alias="esec")
    token_plaintext: str = pydantic.Field(alias="tok")


class _AutoConfigTokenContentV2(pydantic.BaseModel):
    app_id: str = pydantic.Field(alias="aid")
    subscription_id: str = pydantic.Field(alias="sid")
    server_url: str = pydantic.Field(alias="surl")
    endpoint_secret: str = pydantic.Field(alias="esec")
    token_plaintext: str = pydantic.Field(alias="tok")


def _parse_token_payload(token: str, prefix: str) -> bytes:
    if not token.startswith(prefix):
        raise AutoConfigError(_UNSUPPORTED_TOKEN_VERSION)
    b64 = token[len(prefix) :]
    try:
        return base64.b64decode(b64 + "===")
    except Exception as exc:
        raise AutoConfigError("invalid token") from exc


def _decode_autoconfig_token_v1(token: str) -> _AutoConfigTokenContentV1:
    decoded = _parse_token_payload(token, _AUTOCONFIG_TOKEN_PREFIX_V1)
    try:
        return _AutoConfigTokenContentV1.model_validate_json(decoded)
    except Exception as exc:
        raise AutoConfigError("invalid token") from exc


def _decode_autoconfig_token_v2(token: str) -> _AutoConfigTokenContentV2:
    decoded = _parse_token_payload(token, _AUTOCONFIG_TOKEN_PREFIX_V2)
    try:
        return _AutoConfigTokenContentV2.model_validate_json(decoded)
    except Exception as exc:
        raise AutoConfigError("invalid token") from exc


def _decode_autoconfig_token(
    token: str,
) -> t.Union[_AutoConfigTokenContentV1, _AutoConfigTokenContentV2]:
    if token.startswith(_AUTOCONFIG_TOKEN_PREFIX_V1):
        return _decode_autoconfig_token_v1(token)
    if token.startswith(_AUTOCONFIG_TOKEN_PREFIX_V2):
        return _decode_autoconfig_token_v2(token)
    raise AutoConfigError(_UNSUPPORTED_TOKEN_VERSION)


def _sink_in_common_to_polling_destination(sink: SinkInCommon) -> DestinationIn:
    return DestinationIn.model_validate(
        {
            "type": "pollingEndpoint",
            "uid": sink.uid,
            "event_types": sink.event_types,
            "channels": sink.channels,
            "metadata": sink.metadata,
        }
    )


def _destination_out_from_v1_endpoint(endpoint: EndpointOut) -> DestinationOut:
    return DestinationOut(
        id=endpoint.id,
        uid=endpoint.uid,
        status=SinkStatus.DISABLED if endpoint.disabled else SinkStatus.ENABLED,
        current_iterator="",
        created_at=endpoint.created_at,
        updated_at=endpoint.updated_at,
        batch_size=0,
        max_wait_secs=0,
        event_types=endpoint.event_types,
        channels=endpoint.channels,
        metadata=endpoint.metadata,
        type="pollingEndpoint",
        config={},
    )


class AutoConfig:
    _app_id: str
    _endpoint_id: t.Optional[str]
    _autoconfig_id: t.Optional[str]
    _endpoint: EndpointIn
    _webhook: Webhook
    _client: AuthenticatedClient
    _httpx_client: t.Optional[httpx.Client]
    _httpx_async_client: t.Optional[httpx.AsyncClient]

    def __init__(self, token: str, endpoint: EndpointIn) -> None:
        content = _decode_autoconfig_token(token)
        try:
            webhook = Webhook(content.endpoint_secret)
        except Exception as exc:
            raise AutoConfigError("invalid token") from exc

        svix = Svix(
            content.token_plaintext,
            SvixOptions(server_url=content.server_url),
        )

        self._app_id = content.app_id
        self._endpoint = endpoint
        self._webhook = webhook
        self._client = svix._client
        self._httpx_client = None
        self._httpx_async_client = None

        if isinstance(content, _AutoConfigTokenContentV1):
            self._endpoint_id = content.endpoint_id
            self._autoconfig_id = None
        else:
            self._endpoint_id = None
            self._autoconfig_id = content.subscription_id

    def subscribe(self) -> EndpointOut:
        if self._httpx_client is None:
            self._httpx_client = _make_httpx_client(self._client)

        if self._autoconfig_id is not None:
            return EndpointAutoconfig(self._client, self._httpx_client).subscribe(
                self._app_id,
                self._autoconfig_id,
                self._endpoint,
            )

        return EndpointAutoConfigDeprecated(self._client, self._httpx_client).update(
            self._app_id,
            self._endpoint_id or "",
            SubscribeIn(endpoint=self._endpoint),
        )

    async def subscribe_async(self) -> EndpointOut:
        if self._httpx_async_client is None:
            self._httpx_async_client = _make_httpx_async_client(self._client)

        if self._autoconfig_id is not None:
            return await EndpointAutoconfigAsync(
                self._client, self._httpx_async_client
            ).subscribe(
                self._app_id,
                self._autoconfig_id,
                self._endpoint,
            )

        return await EndpointAutoConfigDeprecatedAsync(
            self._client, self._httpx_async_client
        ).update(
            self._app_id,
            self._endpoint_id or "",
            SubscribeIn(endpoint=self._endpoint),
        )

    def verify(self, payload: t.Union[bytes, str], headers: t.Dict[str, str]) -> t.Any:
        return self._webhook.verify(payload, headers)


class AutoConfigConsumer:
    _app_id: str
    _sink_id: t.Optional[str]
    _autoconfig_id: t.Optional[str]
    _sink_in: SinkInCommon
    _client: AuthenticatedClient
    _httpx_client: t.Optional[httpx.Client]
    _httpx_async_client: t.Optional[httpx.AsyncClient]

    def __init__(self, token: str, sink_in: SinkInCommon) -> None:
        content = _decode_autoconfig_token(token)

        svix = Svix(
            content.token_plaintext,
            SvixOptions(server_url=content.server_url),
        )

        self._app_id = content.app_id
        self._sink_in = sink_in
        self._client = svix._client
        self._httpx_client = None
        self._httpx_async_client = None

        if isinstance(content, _AutoConfigTokenContentV1):
            self._sink_id = content.endpoint_id
            self._autoconfig_id = None
        else:
            self._sink_id = None
            self._autoconfig_id = content.subscription_id

    def _subscribe_in(self) -> SubscribeIn:
        return SubscribeIn(
            sink=AutoConfigSinkType(
                type="poller",
                config=self._sink_in,
            ),
        )

    def subscribe(self) -> DestinationOut:
        if self._httpx_client is None:
            self._httpx_client = _make_httpx_client(self._client)

        if self._autoconfig_id is not None:
            destination = DestinationAutoconfig(
                self._client, self._httpx_client
            ).subscribe(
                self._app_id,
                self._autoconfig_id,
                _sink_in_common_to_polling_destination(self._sink_in),
            )
            self._sink_id = destination.id
            return destination

        return _destination_out_from_v1_endpoint(
            EndpointAutoConfigDeprecated(self._client, self._httpx_client).update(
                self._app_id,
                self._sink_id or "",
                self._subscribe_in(),
            )
        )

    async def subscribe_async(self) -> DestinationOut:
        if self._httpx_async_client is None:
            self._httpx_async_client = _make_httpx_async_client(self._client)

        if self._autoconfig_id is not None:
            destination = await DestinationAutoconfigAsync(
                self._client, self._httpx_async_client
            ).subscribe(
                self._app_id,
                self._autoconfig_id,
                _sink_in_common_to_polling_destination(self._sink_in),
            )
            self._sink_id = destination.id
            return destination

        return _destination_out_from_v1_endpoint(
            await EndpointAutoConfigDeprecatedAsync(
                self._client, self._httpx_async_client
            ).update(
                self._app_id,
                self._sink_id or "",
                self._subscribe_in(),
            )
        )

    def receive(
        self,
        consumer_id: str,
        options: MessagePollerv2ConsumerPollOptions = (
            MessagePollerv2ConsumerPollOptions()
        ),
    ) -> PollerV2PollOut:
        if self._sink_id is None:
            self._sink_id = self.subscribe().id
        if self._httpx_client is None:
            self._httpx_client = _make_httpx_client(self._client)

        return MessagePollerv2(self._client, self._httpx_client).consumer_poll(
            self._app_id,
            self._sink_id,
            consumer_id,
            options,
        )

    async def receive_async(
        self,
        consumer_id: str,
        options: MessagePollerv2ConsumerPollOptions = (
            MessagePollerv2ConsumerPollOptions()
        ),
    ) -> PollerV2PollOut:
        if self._sink_id is None:
            self._sink_id = (await self.subscribe_async()).id
        if self._httpx_async_client is None:
            self._httpx_async_client = _make_httpx_async_client(self._client)

        return await MessagePollerv2Async(
            self._client, self._httpx_async_client
        ).consumer_poll(
            self._app_id,
            self._sink_id,
            consumer_id,
            options,
        )

    def commit(
        self,
        consumer_id: str,
        offset: int,
        options: MessagePollerv2ConsumerCommitOptions = (
            MessagePollerv2ConsumerCommitOptions()
        ),
    ) -> None:
        if self._sink_id is None:
            self._sink_id = self.subscribe().id
        if self._httpx_client is None:
            self._httpx_client = _make_httpx_client(self._client)

        MessagePollerv2(self._client, self._httpx_client).consumer_commit(
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
        options: MessagePollerv2ConsumerCommitOptions = (
            MessagePollerv2ConsumerCommitOptions()
        ),
    ) -> None:
        if self._sink_id is None:
            self._sink_id = (await self.subscribe_async()).id
        if self._httpx_async_client is None:
            self._httpx_async_client = _make_httpx_async_client(self._client)

        await MessagePollerv2Async(
            self._client, self._httpx_async_client
        ).consumer_commit(
            self._app_id,
            self._sink_id,
            consumer_id,
            PollerV2CommitIn(offset=offset),
            options,
        )


__all__ = [
    "AutoConfig",
    "AutoConfigConsumer",
    "AutoConfigError",
]
