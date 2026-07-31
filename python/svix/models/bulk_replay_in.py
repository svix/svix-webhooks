# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .message_status import MessageStatus
from .status_code_class import StatusCodeClass


class BulkReplayIn(BaseModel):
    since: datetime

    until: t.Optional[datetime] = None

    event_types: t.Optional[t.List[str]] = None

    channel: t.Optional[str] = None

    tag: t.Optional[str] = None

    status: t.Optional[MessageStatus] = None

    status_code_class: t.Optional[StatusCodeClass] = None
