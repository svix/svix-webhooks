# this file is @generated
import typing as t

from .common import BaseModel
from .polling_endpoint_message_out import PollingEndpointMessageOut


class PollingEndpointOut(BaseModel):
    data: t.List[PollingEndpointMessageOut]

    done: bool

    iterator: str
