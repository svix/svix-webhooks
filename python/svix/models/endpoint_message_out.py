# this file is @generated
import typing as t
from datetime import datetime

from .common import SvixBaseModel
from .message_status import MessageStatus


class EndpointMessageOut(SvixBaseModel):
    channels: t.Optional[t.Set[str]] = None
    """List of free-form identifiers that endpoints can filter by"""
    event_id: t.Optional[str] = None
    """Optional unique identifier for the message"""
    event_type: str
    """The event type's name"""
    id: str
    """The msg's ID"""
    next_attempt: t.Optional[datetime] = None
    payload: t.Dict[str, t.Any]
    status: MessageStatus
    tags: t.Optional[t.Set[str]] = None
    timestamp: datetime
