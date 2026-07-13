# this file is @generated
import typing as t

from .common import BaseModel
from .stream_sink_out import StreamSinkOut


class ListResponseStreamSinkOut(BaseModel):
    data: t.List[StreamSinkOut]

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None

    done: bool
