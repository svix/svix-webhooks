# this file is @generated

from .background_task_data import BackgroundTaskData
from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import SvixBaseModel


class BackgroundTaskOut(SvixBaseModel):
    data: BackgroundTaskData

    id: str

    status: BackgroundTaskStatus

    task: BackgroundTaskType
