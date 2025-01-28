# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .event_type_out import EventTypeOut


class ListResponseEventTypeOut(SvixBaseModel):
    data: t.List[EventTypeOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
