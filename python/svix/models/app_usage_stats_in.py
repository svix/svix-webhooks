# this file is @generated
import typing as t
from datetime import datetime

from .common import SvixBaseModel


class AppUsageStatsIn(SvixBaseModel):
    app_ids: t.Optional[t.Set[str]] = None
    """Specific app IDs or UIDs to aggregate stats for.

    Note that if none of the given IDs or UIDs are resolved, a 422 response will be given."""
    since: datetime
    until: datetime
