# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .message_out import MessageOut


class ListResponseMessageOut(SvixBaseModel):
    data: t.List[MessageOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
