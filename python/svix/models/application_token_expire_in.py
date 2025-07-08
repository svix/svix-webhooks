# this file is @generated
import typing as t

from .common import BaseModel


class ApplicationTokenExpireIn(BaseModel):
    expiry: t.Optional[int] = None
    """How many seconds until the old key is expired."""

    session_ids: t.Optional[t.List[str]] = None
    """An optional list of session ids.

    If any session ids are specified, only Application tokens created with that session id will be expired."""
