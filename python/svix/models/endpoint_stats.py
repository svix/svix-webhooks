# this file is @generated

from .common import BaseModel


class EndpointStats(BaseModel):
    success: int

    pending: int

    sending: int

    fail: int

    canceled: int
