# this file is @generated
import typing as t

from .common import SvixBaseModel
from .endpoint_out import EndpointOut


class ListResponseEndpointOut(SvixBaseModel):
    data: t.List[EndpointOut]
    done: bool
    iterator: str
    prev_iterator: t.Optional[str] = None
