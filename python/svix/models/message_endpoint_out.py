# this file is @generated
import typing as t
from datetime import datetime

from .common import SvixBaseModel
from .message_status import MessageStatus


class MessageEndpointOut(SvixBaseModel):
    channels: t.Optional[t.Set[str]] = None
    """List of message channels this endpoint listens to (omit for all)."""
    created_at: datetime
    description: str
    """An example endpoint name."""
    disabled: t.Optional[bool] = None
    filter_types: t.Optional[t.Set[str]] = None
    id: str
    """The ep's ID"""
    next_attempt: t.Optional[datetime] = None
    rate_limit: t.Optional[int] = None
    status: MessageStatus
    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""
    updated_at: datetime
    url: str
    version: int
