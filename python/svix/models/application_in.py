# this file is @generated
import typing as t

from .common import BaseModel


class ApplicationIn(BaseModel):
    metadata: t.Optional[t.Dict[str, str]] = None

    name: str

    rate_limit: t.Optional[int] = None

    uid: t.Optional[str] = None
    """Optional unique identifier for the application."""
