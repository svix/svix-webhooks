# this file is @generated
import typing as t

from .common import BaseModel


class ApiTokenExpireIn(BaseModel):
    expiry: t.Optional[int] = None
    """How many seconds until the old key is expired."""
