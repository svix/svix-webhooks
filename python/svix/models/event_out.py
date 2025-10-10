# this file is @generated
from datetime import datetime

from .common import BaseModel


class EventOut(BaseModel):
    event_type: str
    """The event type's name"""

    payload: str

    timestamp: datetime
