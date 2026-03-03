# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class MsgIn(BaseModel):
    value: bytes

    headers: t.Optional[t.Dict[str, str]] = None

    key: t.Optional[str] = None
    """Optional partition key. Messages with the same key are routed to the same partition."""
