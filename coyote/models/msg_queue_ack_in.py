# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgQueueAckIn(BaseModel):
    namespace: t.Optional[str] = None

    msg_ids: t.List[str] = Field(alias="msg_ids")


class _MsgQueueAckIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str = Field(alias="consumer_group")

    msg_ids: t.List[str] = Field(alias="msg_ids")
