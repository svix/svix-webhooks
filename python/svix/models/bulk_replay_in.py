# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .message_status import MessageStatus
from .status_code_class import StatusCodeClass


class BulkReplayIn(BaseModel):
    channel: t.Optional[str] = None

    event_types: t.Optional[t.List[str]] = None

    since: datetime

    status: t.Optional[MessageStatus] = None

    status_code_class: t.Optional[StatusCodeClass] = None

    tag: t.Optional[str] = None

    until: t.Optional[datetime] = None
