# this file is @generated

from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import BaseModel


class RecoverOut(BaseModel):
    id: str

    status: BackgroundTaskStatus

    task: BackgroundTaskType
