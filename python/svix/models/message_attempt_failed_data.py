# this file is @generated
from datetime import datetime

from .common import BaseModel


class MessageAttemptFailedData(BaseModel):
    id: str
    """The MessageAttempt's ID."""

    response_status_code: int

    timestamp: datetime
