# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class PollingEndpointMessageOut(BaseModel):
    """The MessageOut equivalent of polling endpoint"""

    channels: t.Optional[t.List[str]] = None
    """List of free-form identifiers that endpoints can filter by"""

    deliver_at: t.Optional[datetime] = None

    event_id: t.Optional[str] = None
    """Optional unique identifier for the message"""

    event_type: str
    """The event type's name"""

    headers: t.Optional[t.Dict[str, str]] = None

    id: str
    """The Message's ID."""

    payload: t.Dict[str, t.Any]

    tags: t.Optional[t.List[str]] = None

    timestamp: datetime
