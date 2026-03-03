# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class CacheGetOut(BaseModel):
    key: str

    expiry: t.Optional[datetime] = None
    """Time of expiry"""

    value: bytes
