# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel
from .message_status import MessageStatus


class EndpointMessageOut(SvixBaseModel):
    """A model containing information on a given message plus additional fields on the last attempt for that message."""

    channels: t.Optional[t.List[str]] = None
    """List of free-form identifiers that endpoints can filter by"""

    event_id: t.Optional[str] = Field(default=None, alias="eventId")
    """Optional unique identifier for the message"""

    event_type: str = Field(alias="eventType")
    """The event type's name"""

    id: str
    """The msg's ID"""

    next_attempt: t.Optional[datetime] = Field(default=None, alias="nextAttempt")

    payload: t.Dict[str, t.Any]

    status: MessageStatus

    tags: t.Optional[t.List[str]] = None

    timestamp: datetime
