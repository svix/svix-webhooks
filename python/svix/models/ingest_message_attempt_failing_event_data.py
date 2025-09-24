# this file is @generated
import typing as t

from .common import BaseModel
from .message_attempt_failed_data import MessageAttemptFailedData


class IngestMessageAttemptFailingEventData(BaseModel):
    """Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "ingest.message.attempt.exhausted" type, after it's failed four times as a "ingest.message.attempt.failing" event, or after it's recovered as a "ingest.message.attempt.recovered" event."""

    endpoint_id: str
    """The Endpoint's ID."""

    last_attempt: MessageAttemptFailedData

    msg_event_id: t.Optional[str] = None
    """The Message's UID."""

    msg_id: str
    """The Message's ID."""

    source_id: str
    """The Source's ID."""
