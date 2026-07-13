# this file is @generated
import typing as t

from .common import BaseModel


class EndpointUpsertIn(BaseModel):
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

    metadata: t.Optional[t.Dict[str, str]] = None
