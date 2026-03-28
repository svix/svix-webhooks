# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgQueueRedriveDlqIn(BaseModel):
    namespace: t.Optional[str] = None


class _MsgQueueRedriveDlqIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str = Field(alias="consumer_group")
