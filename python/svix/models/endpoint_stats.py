# this file is @generated

from .common import BaseModel


class EndpointStats(BaseModel):
    fail: int

    pending: int

    sending: int

    success: int
