# this file is @generated
import typing as t
from datetime import datetime

from pydantic import Field

from .common import SvixBaseModel


class MessageOut(SvixBaseModel):
    channels: t.Optional[t.List[str]] = None
    """List of free-form identifiers that endpoints can filter by"""

    event_id: t.Optional[str] = Field(default=None, alias="eventId")
    """Optional unique identifier for the message"""

    event_type: str = Field(alias="eventType")
    """The event type's name"""

    id: str
    """The msg's ID"""

    payload: t.Dict[str, t.Any]

    tags: t.Optional[t.List[str]] = None

    timestamp: datetime
