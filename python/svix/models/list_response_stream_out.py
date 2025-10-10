# this file is @generated
import typing as t

from .common import BaseModel
from .stream_out import StreamOut


class ListResponseStreamOut(BaseModel):
    data: t.List[StreamOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
