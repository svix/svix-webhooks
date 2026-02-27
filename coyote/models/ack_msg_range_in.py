# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AckMsgRangeIn(BaseModel):
    consumer_group: str = Field(alias="consumer_group")

    max_msg_id: int = Field(alias="max_msg_id")

    min_msg_id: t.Optional[int] = Field(default=None, alias="min_msg_id")

    name: str
