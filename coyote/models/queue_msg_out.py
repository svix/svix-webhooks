# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class QueueMsgOut(BaseModel):
    msg_id: str = Field(alias="msg_id")

    value: bytes

    headers: t.Optional[t.Dict[str, str]] = None

    timestamp: datetime

    scheduled_at: t.Optional[datetime] = Field(default=None, alias="scheduled_at")
