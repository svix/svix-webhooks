# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class AuthTokenVerifyIn(BaseModel):
    namespace: t.Optional[str] = None

    token: str
