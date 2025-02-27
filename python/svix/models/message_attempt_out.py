# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .message_attempt_trigger_type import MessageAttemptTriggerType
from .message_out import MessageOut
from .message_status import MessageStatus


class MessageAttemptOut(BaseModel):
    endpoint_id: str
    """The Endpoint's ID."""

    id: str
    """The MessageAttempt's ID."""

    msg: t.Optional[MessageOut] = None

    msg_id: str
    """The Message's ID."""

    response: str

    response_duration_ms: int
    """Response duration in milliseconds."""

    response_status_code: int

    status: MessageStatus

    timestamp: datetime

    trigger_type: MessageAttemptTriggerType

    url: str
