# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .msg_out import MsgOut


class FetchFromStreamOut(BaseModel):
    msgs: t.List[MsgOut]
