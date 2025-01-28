# this file is @generated
import typing as t

from pydantic import Field

from .background_task_out import BackgroundTaskOut
from .common import SvixBaseModel


class ListResponseBackgroundTaskOut(SvixBaseModel):
    data: t.List[BackgroundTaskOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
