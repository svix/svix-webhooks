# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class CacheGetOut(BaseModel):
    expiry: t.Optional[datetime] = None
    """Time of expiry"""

    key: str

    value: t.List[int]
