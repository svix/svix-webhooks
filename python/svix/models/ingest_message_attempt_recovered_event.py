# this file is @generated

from .common import BaseModel
from .ingest_message_attempt_recovered_event_data import (
    IngestMessageAttemptRecoveredEventData,
)


class IngestMessageAttemptRecoveredEvent(BaseModel):
    """Sent on a successful dispatch after an earlier failure op webhook has already been sent."""

    data: IngestMessageAttemptRecoveredEventData

    type: str
