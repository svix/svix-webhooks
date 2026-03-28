# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .auth_token_out import AuthTokenOut


class ListResponseAuthTokenOut(BaseModel):
    data: t.List[AuthTokenOut]

    iterator: t.Optional[str] = None

    prev_iterator: t.Optional[str] = Field(default=None, alias="prev_iterator")

    done: bool
