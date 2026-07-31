# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class PollingEndpointMessageOut(BaseModel):
    """The MessageOut equivalent of polling endpoint"""

    headers: t.Optional[t.Dict[str, str]] = None

    event_id: t.Optional[str] = None
    """Optional unique identifier for the message"""

    event_type: str
    """The event type's name"""

    payload: t.Dict[str, t.Any]

    channels: t.Optional[t.List[str]] = None
    """List of free-form identifiers that endpoints can filter by"""

    id: str
    """The Message's ID."""

    timestamp: datetime

    tags: t.Optional[t.List[str]] = None

    deliver_at: t.Optional[datetime] = None
