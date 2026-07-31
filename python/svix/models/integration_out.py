# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class IntegrationOut(BaseModel):
    name: str

    id: str
    """The Integration's ID."""

    created_at: datetime

    updated_at: datetime

    feature_flags: t.Optional[t.List[str]] = None
    """The set of feature flags the integration has access to."""
