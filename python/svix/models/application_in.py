# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel


class ApplicationIn(SvixBaseModel):
    metadata: t.Optional[t.Dict[str, str]] = None

    name: str

    rate_limit: t.Optional[int] = Field(default=None, alias="rateLimit")

    uid: t.Optional[str] = None
    """Optional unique identifier for the application."""
