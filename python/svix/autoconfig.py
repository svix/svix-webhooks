import base64
import typing as t

import pydantic

from .api.client import AuthenticatedClient
from .api.svix import Svix, SvixOptions
from .api_internal.endpoint_auto_config import (
    EndpointAutoConfig,
    EndpointAutoConfigAsync,
)
from .models import EndpointIn, EndpointOut, SubscribeIn
from .webhooks import Webhook

_AUTOCONFIG_TOKEN_PREFIX_V1 = "auto_v1_"


class AutoConfigError(Exception):
    """Raised when an autoconfig token cannot be decoded or validated."""


class _AutoConfigTokenContentV1(pydantic.BaseModel):
    """Payload embedded in a v1 autoconfig token (short JSON keys)."""

    app_id: str = pydantic.Field(alias="aid")
    endpoint_id: str = pydantic.Field(alias="eid")
    server_url: str = pydantic.Field(alias="surl")
    endpoint_secret: str = pydantic.Field(alias="esec")
    token_plaintext: str = pydantic.Field(alias="tok")


def _decode_autoconfig_token_v1(token: str) -> _AutoConfigTokenContentV1:
    if not token.startswith(_AUTOCONFIG_TOKEN_PREFIX_V1):
        raise AutoConfigError("invalid token")
    b64 = token[len(_AUTOCONFIG_TOKEN_PREFIX_V1) :]
    try:
        decoded = base64.b64decode(b64)
    except Exception as exc:
        raise AutoConfigError("invalid token") from exc
    try:
        return _AutoConfigTokenContentV1.model_validate_json(decoded)
    except Exception as exc:
        raise AutoConfigError("invalid token") from exc


class AutoConfig:
    _app_id: str
    _endpoint_id: str
    _endpoint: EndpointIn
    _webhook: Webhook
    _client: AuthenticatedClient

    def __init__(self, token: str, endpoint: EndpointIn) -> None:
        content = _decode_autoconfig_token_v1(token)
        try:
            webhook = Webhook(content.endpoint_secret)
        except Exception as exc:
            raise AutoConfigError("invalid token") from exc

        svix = Svix(
            content.token_plaintext,
            SvixOptions(server_url=content.server_url),
        )

        self._app_id = content.app_id
        self._endpoint_id = content.endpoint_id
        self._endpoint = endpoint
        self._webhook = webhook
        self._client = svix._client

    def subscribe(self) -> EndpointOut:
        return EndpointAutoConfig(self._client).update(
            self._app_id,
            self._endpoint_id,
            SubscribeIn(endpoint=self._endpoint),
        )

    async def subscribe_async(self) -> EndpointOut:
        return await EndpointAutoConfigAsync(self._client).update(
            self._app_id,
            self._endpoint_id,
            SubscribeIn(endpoint=self._endpoint),
        )

    def verify(self, payload: t.Union[bytes, str], headers: t.Dict[str, str]) -> t.Any:
        return self._webhook.verify(payload, headers)


__all__ = ["AutoConfig", "AutoConfigError"]
