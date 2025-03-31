# this file is @generated
import typing as t

from .api_token_censored_out import ApiTokenCensoredOut
from .common import BaseModel


class ListResponseApiTokenCensoredOut(BaseModel):
    data: t.List[ApiTokenCensoredOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
