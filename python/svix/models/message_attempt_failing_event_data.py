# this file is @generated
import typing as t

from .common import BaseModel
from .message_attempt_failed_data import MessageAttemptFailedData


class MessageAttemptFailingEventData(BaseModel):
    """Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing" event."""

    app_id: str
    """The Application's ID."""

    app_uid: t.Optional[str] = None
    """The Application's UID."""

    endpoint_id: str
    """The Endpoint's ID."""

    last_attempt: MessageAttemptFailedData

    msg_event_id: t.Optional[str] = None
    """The Message's UID."""

    msg_id: str
    """The Message's ID."""
