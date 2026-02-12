# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .message_status import MessageStatus
from .message_status_text import MessageStatusText


class MessageEndpointOut(BaseModel):
    channels: t.Optional[t.List[str]] = None
    """List of message channels this endpoint listens to (omit for all)."""

    created_at: datetime

    description: str
    """An example endpoint name."""

    disabled: t.Optional[bool] = None

    filter_types: t.Optional[t.List[str]] = None

    id: str
    """The Endpoint's ID."""

    next_attempt: t.Optional[datetime] = None

    rate_limit: t.Optional[int] = None
    """Deprecated, use `throttleRate` instead."""

    status: MessageStatus

    status_text: MessageStatusText

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this endpoint. Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    updated_at: datetime

    url: str

    version: int
