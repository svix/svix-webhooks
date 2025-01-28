# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel


class ApplicationPatch(SvixBaseModel):
    metadata: t.Optional[t.Dict[str, str]] = None

    name: t.Optional[str] = None

    rate_limit: t.Optional[int] = Field(default=None, alias="rateLimit")

    uid: t.Optional[str] = None
    """The app's UID"""
