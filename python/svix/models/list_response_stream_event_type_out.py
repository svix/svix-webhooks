# this file is @generated
import typing as t

from .common import BaseModel
from .stream_event_type_out import StreamEventTypeOut


class ListResponseStreamEventTypeOut(BaseModel):
    data: t.List[StreamEventTypeOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
