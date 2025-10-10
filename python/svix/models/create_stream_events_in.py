# this file is @generated
import typing as t

from .common import BaseModel
from .event_in import EventIn
from .stream_in import StreamIn


class CreateStreamEventsIn(BaseModel):
    events: t.List[EventIn]

    stream: t.Optional[StreamIn] = None
    """Optionally creates a new Stream alongside the events.

    If the stream id or uid that is used in the path already exists, this argument is ignored."""
