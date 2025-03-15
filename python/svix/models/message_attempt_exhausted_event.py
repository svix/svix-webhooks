# this file is @generated

from .common import BaseModel
from .message_attempt_exhausted_event_data import MessageAttemptExhaustedEventData


class MessageAttemptExhaustedEvent(BaseModel):
    """Sent when a message delivery has failed (all of the retry attempts have been exhausted)."""

    data: MessageAttemptExhaustedEventData

    type: str
