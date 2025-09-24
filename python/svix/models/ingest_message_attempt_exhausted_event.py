# this file is @generated

from .common import BaseModel
from .ingest_message_attempt_exhausted_event_data import (
    IngestMessageAttemptExhaustedEventData,
)


class IngestMessageAttemptExhaustedEvent(BaseModel):
    """Sent when a message delivery has failed (all of the retry attempts have been exhausted)."""

    data: IngestMessageAttemptExhaustedEventData

    type: str
