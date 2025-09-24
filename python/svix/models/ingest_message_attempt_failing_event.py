# this file is @generated

from .common import BaseModel
from .ingest_message_attempt_failing_event_data import (
    IngestMessageAttemptFailingEventData,
)


class IngestMessageAttemptFailingEvent(BaseModel):
    """Sent after a message has been failing for a few times.
    It's sent on the fourth failure. It complements `ingest.message.attempt.exhausted` which is sent after the last failure."""

    data: IngestMessageAttemptFailingEventData

    type: str
