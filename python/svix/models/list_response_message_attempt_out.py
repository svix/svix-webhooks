# this file is @generated
import typing as t

from .common import BaseModel
from .message_attempt_out import MessageAttemptOut


class ListResponseMessageAttemptOut(BaseModel):
    data: t.List[MessageAttemptOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
