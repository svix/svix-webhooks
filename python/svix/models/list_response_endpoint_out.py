# this file is @generated
import typing as t

from .common import BaseModel
from .endpoint_out import EndpointOut


class ListResponseEndpointOut(BaseModel):
    data: t.List[EndpointOut]

    done: bool

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None
