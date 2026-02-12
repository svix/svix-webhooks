# this file is @generated
import typing as t

from .common import BaseModel


class EndpointPatch(BaseModel):
    channels: t.Optional[t.List[str]] = None

    description: t.Optional[str] = None

    disabled: t.Optional[bool] = None

    filter_types: t.Optional[t.List[str]] = None

    metadata: t.Optional[t.Dict[str, str]] = None

    rate_limit: t.Optional[int] = None
    """Deprecated, use `throttleRate` instead."""

    secret: t.Optional[str] = None
    """The endpoint's verification secret.

    Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    It is recommended to not set this and let the server generate the secret."""

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this endpoint.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """The Endpoint's UID."""

    url: t.Optional[str] = None

    version: t.Optional[int] = None
