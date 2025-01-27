# this file is @generated
import typing as t

from .common import SvixBaseModel


class ApplicationPatch(SvixBaseModel):
    metadata: t.Optional[t.Dict[str, str]] = None
    name: t.Optional[str] = None
    rate_limit: t.Optional[int] = None
    uid: t.Optional[str] = None
    """The app's UID"""
