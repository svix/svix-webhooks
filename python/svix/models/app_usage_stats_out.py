# this file is @generated
import typing as t

from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import BaseModel


class AppUsageStatsOut(BaseModel):
    id: str
    """The QueueBackgroundTask's ID."""

    status: BackgroundTaskStatus

    task: BackgroundTaskType

    unresolved_app_ids: t.List[str]
    """Any app IDs or UIDs received in the request that weren't found.

    Stats will be produced for all the others."""
