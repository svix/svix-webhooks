# this file is @generated

from .common import BaseModel


class EndpointStats(BaseModel):
    canceled: int

    fail: int

    pending: int

    sending: int

    success: int
