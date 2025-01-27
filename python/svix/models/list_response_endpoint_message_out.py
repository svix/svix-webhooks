# this file is @generated
import typing as t

from .common import SvixBaseModel
from .endpoint_message_out import EndpointMessageOut


class ListResponseEndpointMessageOut(SvixBaseModel):
    data: t.List[EndpointMessageOut]
    done: bool
    iterator: str
    prev_iterator: t.Optional[str] = None
