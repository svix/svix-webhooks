# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .message_status import MessageStatus


class BulkReplayIn(BaseModel):
    channel: t.Optional[str] = None

    event_types: t.Optional[t.List[str]] = None

    since: datetime

    status: t.Optional[MessageStatus] = None

    tag: t.Optional[str] = None

    until: t.Optional[datetime] = None
