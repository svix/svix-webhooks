# this file is @generated

from .common import BaseModel
from .message_attempt_recovered_event_data import MessageAttemptRecoveredEventData


class MessageAttemptRecoveredEvent(BaseModel):
    """Sent on a successful dispatch after an earlier failure op webhook has already been sent."""

    data: MessageAttemptRecoveredEventData

    type: str
