# this file is @generated
import typing as t

from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import BaseModel


class BackgroundTaskFinishedEvent2(BaseModel):
    data: t.Dict[str, t.Any]

    status: BackgroundTaskStatus

    task: BackgroundTaskType

    task_id: str
    """The QueueBackgroundTask's ID."""
