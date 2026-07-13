# this file is @generated
import typing as t

from .common import BaseModel


class EndpointIn(BaseModel):
    description: t.Optional[str] = None

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this endpoint.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    url: str

    disabled: t.Optional[bool] = None

    filter_types: t.Optional[t.List[str]] = None

    channels: t.Optional[t.List[str]] = None
    """List of message channels this endpoint listens to (omit for all)."""

    secret: t.Optional[str] = None
    """The endpoint's verification secret.

    Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
    It is recommended to not set this and let the server generate the secret."""

    metadata: t.Optional[t.Dict[str, str]] = None

    headers: t.Optional[t.Dict[str, str]] = None
