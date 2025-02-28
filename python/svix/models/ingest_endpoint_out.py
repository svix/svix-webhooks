# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel


class IngestEndpointOut(BaseModel):
    created_at: datetime

    description: str
    """An example endpoint name."""

    disabled: t.Optional[bool] = None

    id: str
    """The Endpoint's ID."""

    metadata: t.Dict[str, str]

    rate_limit: t.Optional[int] = None

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    updated_at: datetime

    url: str
