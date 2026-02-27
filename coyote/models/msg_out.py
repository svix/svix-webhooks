# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel


class MsgOut(BaseModel):
    headers: t.Dict[str, str]

    id: int

    payload: t.List[int]

    timestamp: datetime
