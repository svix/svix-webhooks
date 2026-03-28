# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class AuthTokenOut(BaseModel):
    id: str

    name: str

    created: datetime

    updated: datetime

    expiry: t.Optional[datetime] = None

    metadata: t.Dict[str, str]

    owner_id: str = Field(alias="owner_id")

    scopes: t.List[str]

    enabled: bool
    """Whether this token is currently enabled."""
