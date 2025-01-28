# this file is @generated

from .common import SvixBaseModel


class EndpointStats(SvixBaseModel):
    fail: int

    pending: int

    sending: int

    success: int
