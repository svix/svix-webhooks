# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .auth_token_out import AuthTokenOut


class AuthTokenVerifyOut(BaseModel):
    token: t.Optional[AuthTokenOut] = None
