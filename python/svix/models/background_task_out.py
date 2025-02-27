# this file is @generated
import typing as t

from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import BaseModel


class BackgroundTaskOut(BaseModel):
    data: t.Dict[str, t.Any]

    id: str
    """The QueueBackgroundTask's ID."""

    status: BackgroundTaskStatus

    task: BackgroundTaskType
