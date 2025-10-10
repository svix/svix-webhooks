# this file is @generated

from .common import BaseModel


class EventIn(BaseModel):
    event_type: str
    """The event type's name"""

    payload: str
