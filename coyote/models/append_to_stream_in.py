# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_in import MsgIn


class AppendToStreamIn(BaseModel):
    msgs: t.List[MsgIn]

    name: str
