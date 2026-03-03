# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgPublishOutTopic(BaseModel):
    topic: str

    start_offset: int = Field(alias="start_offset")

    offset: int
