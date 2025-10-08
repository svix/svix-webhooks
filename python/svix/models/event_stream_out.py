# this file is @generated
import typing as t

from .common import BaseModel
from .event_out import EventOut


class EventStreamOut(BaseModel):
    data: t.List[EventOut]

    done: bool

    iterator: str
