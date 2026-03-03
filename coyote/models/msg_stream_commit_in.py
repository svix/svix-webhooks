# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgStreamCommitIn(BaseModel):
    offset: int


class _MsgStreamCommitIn(BaseModel):
    topic: str

    consumer_group: str = Field(alias="consumer_group")

    offset: int
