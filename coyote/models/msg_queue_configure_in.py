# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgQueueConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    retry_schedule: t.Optional[t.List[int]] = Field(
        default=None, alias="retry_schedule"
    )

    dlq_topic: t.Optional[str] = Field(default=None, alias="dlq_topic")


class _MsgQueueConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str = Field(alias="consumer_group")

    retry_schedule: t.Optional[t.List[int]] = Field(
        default=None, alias="retry_schedule"
    )

    dlq_topic: t.Optional[str] = Field(default=None, alias="dlq_topic")
