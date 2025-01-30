# this file is @generated
import typing as t

from .common import BaseModel
from .endpoint_message_out import EndpointMessageOut


class ListResponseEndpointMessageOut(BaseModel):
    data: t.List[EndpointMessageOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
