# this file is @generated
import typing as t

from .common import BaseModel


class CreateAutoConfigSubscriptionIn(BaseModel):
    feature_flags: t.Optional[t.List[str]] = None
    """The set of feature flags the created token will have access to.

    When omitted or empty, the token inherits the calling token's feature flags.
    When set, these flags are used instead. An application token may only grant a subset of its own flags."""
