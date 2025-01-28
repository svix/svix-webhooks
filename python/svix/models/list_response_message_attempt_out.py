# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .message_attempt_out import MessageAttemptOut


class ListResponseMessageAttemptOut(SvixBaseModel):
    data: t.List[MessageAttemptOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
