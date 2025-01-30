# this file is @generated
import typing as t

from .background_task_out import BackgroundTaskOut
from .common import BaseModel


class ListResponseBackgroundTaskOut(BaseModel):
    data: t.List[BackgroundTaskOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
