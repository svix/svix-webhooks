# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class KvGetOut(BaseModel):
    expiry: t.Optional[datetime] = None
    """Time of expiry"""

    value: t.Optional[bytes] = None

    version: int
    """Opaque version token for optimistic concurrency control.
    Pass as `version` in a subsequent `set` to perform a conditional write."""
