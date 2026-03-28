# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_in import MsgIn


class MsgPublishIn(BaseModel):
    namespace: t.Optional[str] = None

    msgs: t.List[MsgIn]


class _MsgPublishIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    msgs: t.List[MsgIn]
