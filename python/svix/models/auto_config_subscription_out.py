# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .status import Status


class AutoConfigSubscriptionOut(BaseModel):
    created_at: datetime

    token_censored: str

    id: str
    """The AutoConfigSubscription's ID."""

    endp_id: t.Optional[str] = None
    """The Endpoint's ID."""

    dest_id: t.Optional[str] = None
    """The StreamSink's ID."""

    status: Status
