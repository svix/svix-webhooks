# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgTopicConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    partitions: int


class _MsgTopicConfigureIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    partitions: int
