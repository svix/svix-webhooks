# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class StreamMsgOut(BaseModel):
    offset: int

    topic: str

    value: bytes

    headers: t.Optional[t.Dict[str, str]] = None

    timestamp: datetime

    scheduled_at: t.Optional[datetime] = Field(default=None, alias="scheduled_at")
