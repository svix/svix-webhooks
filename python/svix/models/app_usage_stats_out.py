# this file is @generated
import typing as t

from .background_task_status import BackgroundTaskStatus
from .background_task_type import BackgroundTaskType
from .common import SvixBaseModel


class AppUsageStatsOut(SvixBaseModel):
    id: str
    status: BackgroundTaskStatus
    task: BackgroundTaskType
    unresolved_app_ids: t.Set[str]
    """Any app IDs or UIDs received in the request that weren't found.

    Stats will be produced for all the others."""
