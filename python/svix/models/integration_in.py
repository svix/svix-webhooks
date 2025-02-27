# this file is @generated
import typing as t

from .common import BaseModel


class IntegrationIn(BaseModel):
    feature_flags: t.Optional[t.List[str]] = None
    """The set of feature flags the integration will have access to."""

    name: str
