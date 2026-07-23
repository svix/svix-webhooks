# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class EndpointOut(BaseModel):
    id: str
    """The Endpoint's ID."""

    metadata: t.Dict[str, str]

    description: str
    """An example endpoint name."""

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

    created_at: datetime

    updated_at: datetime
