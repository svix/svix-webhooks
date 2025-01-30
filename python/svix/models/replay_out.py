# this file is @generated

from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import BaseModel


class ReplayOut(BaseModel):
    id: str

    status: BackgroundTaskStatus

    task: BackgroundTaskType
