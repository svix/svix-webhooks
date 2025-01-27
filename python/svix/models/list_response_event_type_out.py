# this file is @generated
import typing as t

from .common import SvixBaseModel
from .event_type_out import EventTypeOut


class ListResponseEventTypeOut(SvixBaseModel):
    data: t.List[EventTypeOut]
    done: bool
    iterator: str
    prev_iterator: t.Optional[str] = None
