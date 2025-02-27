# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class MessageOut(BaseModel):
    channels: t.Optional[t.List[str]] = None
    """List of free-form identifiers that endpoints can filter by"""

    event_id: t.Optional[str] = None
    """Optional unique identifier for the message"""

    event_type: str
    """The event type's name"""

    id: str
    """The Message's ID."""

    payload: t.Dict[str, t.Any]

    tags: t.Optional[t.List[str]] = None

    timestamp: datetime
