# this file is @generated
import typing as t

from .common import BaseModel


class IngestSourceConsumerPortalAccessIn(BaseModel):
    expiry: t.Optional[int] = None
    """How long the token will be valid for, in seconds.

    Valid values are between 1 hour and 7 days. The default is 7 days."""

    read_only: t.Optional[bool] = None
    """Whether the app portal should be in read-only mode."""
