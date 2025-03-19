# this file is @generated

from .common import BaseModel
from .message_attempt_failing_event_data import MessageAttemptFailingEventData


class MessageAttemptFailingEvent(BaseModel):
    """Sent after a message has been failing for a few times.
    It's sent on the fourth failure. It complements `message.attempt.exhausted` which is sent after the last failure."""

    data: MessageAttemptFailingEventData

    type: str
