# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .message_attempt_trigger_type import MessageAttemptTriggerType
from .message_out import MessageOut
from .message_status import MessageStatus
from .message_status_text import MessageStatusText


class MessageAttemptOut(BaseModel):
    url: str

    response: str

    response_status_code: int

    response_duration_ms: int
    """Response duration in milliseconds."""

    status: MessageStatus

    status_text: MessageStatusText

    trigger_type: MessageAttemptTriggerType

    msg_id: str
    """The Message's ID."""

    endpoint_id: str
    """The Endpoint's ID."""

    id: str
    """The MessageAttempt's ID."""

    timestamp: datetime

    msg: t.Optional[MessageOut] = None
