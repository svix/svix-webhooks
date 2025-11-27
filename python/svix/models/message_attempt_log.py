# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .http_attempt_times import HttpAttemptTimes
from .message_status import MessageStatus


class MessageAttemptLog(BaseModel):
    app_id: str
    """The Application's ID."""

    app_uid: t.Optional[str] = None
    """The Application's UID."""

    attempt_count: int

    attempt_end: datetime

    attempt_id: str
    """The MessageAttempt's ID."""

    attempt_start: datetime

    endpoint_id: str
    """The Endpoint's ID."""

    event_type: t.Optional[str] = None
    """The event type's name"""

    http_times: t.Optional[HttpAttemptTimes] = None

    msg_created: datetime

    msg_event_id: t.Optional[str] = None
    """The Message's UID."""

    msg_id: str
    """The Message's ID."""

    org_id: str
    """The Environment's ID."""

    response_status_code: int

    status: MessageStatus
