# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class IntegrationOut(BaseModel):
    created_at: datetime

    feature_flags: t.Optional[t.List[str]] = None
    """The set of feature flags the integration has access to."""

    id: str
    """The Integration's ID."""

    name: str

    updated_at: datetime
