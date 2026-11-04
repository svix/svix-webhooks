# this file is @generated

from .common import BaseModel


class EndpointAttemptStats(BaseModel):
    success: int

    fail: int

    canceled: int
