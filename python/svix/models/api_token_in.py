# this file is @generated
import typing as t

from .common import BaseModel


class ApiTokenIn(BaseModel):
    name: str

    scopes: t.Optional[t.List[str]] = None
