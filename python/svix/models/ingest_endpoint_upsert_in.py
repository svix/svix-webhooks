# this file is @generated
import typing as t

from .common import BaseModel


class IngestEndpointUpsertIn(BaseModel):
    url: str

    description: t.Optional[str] = None

    throttle_rate: t.Optional[int] = None
    """Maximum messages per second to send to this endpoint.

    Outgoing messages will be throttled to this rate."""

    uid: t.Optional[str] = None
    """Optional unique identifier for the endpoint."""

    disabled: t.Optional[bool] = None

    metadata: t.Optional[t.Dict[str, str]] = None
