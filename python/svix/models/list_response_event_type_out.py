# this file is @generated
import typing as t

from .common import BaseModel
from .event_type_out import EventTypeOut


class ListResponseEventTypeOut(BaseModel):
    data: t.List[EventTypeOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
