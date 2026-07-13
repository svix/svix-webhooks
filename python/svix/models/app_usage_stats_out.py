# this file is @generated
import typing as t
from datetime import datetime

from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import BaseModel


class AppUsageStatsOut(BaseModel):
    unresolved_app_ids: t.List[str]
    """Any app IDs or UIDs received in the request that weren't found.

    Stats will be produced for all the others."""

    id: str
    """The QueueBackgroundTask's ID."""

    status: BackgroundTaskStatus

    task: BackgroundTaskType

    updated_at: datetime
