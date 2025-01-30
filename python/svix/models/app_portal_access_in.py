# this file is @generated
import typing as t

from .application_in import ApplicationIn
from .common import BaseModel


class AppPortalAccessIn(BaseModel):
    application: t.Optional[ApplicationIn] = None
    """Optionally creates a new application while generating the access link.

    If the application id or uid that is used in the path already exists, this argument is ignored."""

    expiry: t.Optional[int] = None
    """How long the token will be valid for, in seconds.

    Valid values are between 1 hour and 7 days. The default is 7 days."""

    feature_flags: t.Optional[t.List[str]] = None
    """The set of feature flags the created token will have access to."""

    read_only: t.Optional[bool] = None
    """Whether the app portal should be in read-only mode."""
