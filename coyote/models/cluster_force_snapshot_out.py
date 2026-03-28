# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class ClusterForceSnapshotOut(BaseModel):
    snapshot_time: datetime = Field(alias="snapshot_time")
    """The wall-clock time at which the snapshot was initiated"""

    snapshot_log_index: int = Field(alias="snapshot_log_index")
    """The log index at which the snapshot was initiated"""

    snapshot_id: t.Optional[str] = Field(default=None, alias="snapshot_id")
    """If this is `null`, the snapshot is still building in the background"""
