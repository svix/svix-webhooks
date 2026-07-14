# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .message_status import MessageStatus
from .message_status_text import MessageStatusText


class MessageEndpointOut(BaseModel):
    id: str
    """The Endpoint's ID."""

    status: MessageStatus

    status_text: MessageStatusText

    next_attempt: t.Optional[datetime] = None

    description: str
    """An example endpoint name."""

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this endpoint.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    url: str

    disabled: t.Optional[bool] = None

    event_types: t.Optional[t.List[str]] = None

    channels: t.Optional[t.List[str]] = None
    """List of message channels this endpoint listens to (omit for all)."""

    created_at: datetime

    updated_at: datetime
