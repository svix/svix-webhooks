# this file is @generated
import typing as t

from .common import BaseModel
from .message_out import MessageOut


class ListResponseMessageOut(BaseModel):
    data: t.List[MessageOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
