# this file is @generated
import typing as t

from .common import BaseModel
from .message_endpoint_out import MessageEndpointOut


class ListResponseMessageEndpointOut(BaseModel):
    data: t.List[MessageEndpointOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
