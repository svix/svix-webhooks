# this file is @generated
import typing as t

from .common import BaseModel


class EndpointPatch(BaseModel):
    description: t.Optional[str] = None

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this endpoint.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """The Endpoint's UID."""

    url: t.Optional[str] = None

    disabled: t.Optional[bool] = None

    event_types: t.Optional[t.List[str]] = None

    channels: t.Optional[t.List[str]] = None

    metadata: t.Optional[t.Dict[str, str]] = None
