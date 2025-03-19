# this file is @generated

from .background_task_finished_event2 import BackgroundTaskFinishedEvent2
from .common import BaseModel


class BackgroundTaskFinishedEvent(BaseModel):
    """Sent when a background task is finished."""

    data: BackgroundTaskFinishedEvent2

    type: str
