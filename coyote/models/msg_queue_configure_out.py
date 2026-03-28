# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgQueueConfigureOut(BaseModel):
    retry_schedule: t.List[int] = Field(alias="retry_schedule")

    dlq_topic: t.Optional[str] = Field(default=None, alias="dlq_topic")
