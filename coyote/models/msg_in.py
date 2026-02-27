# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgIn(BaseModel):
    headers: t.Optional[t.Dict[str, str]] = None

    payload: t.List[int]
