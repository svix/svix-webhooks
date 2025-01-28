# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .message_endpoint_out import MessageEndpointOut


class ListResponseMessageEndpointOut(SvixBaseModel):
    data: t.List[MessageEndpointOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
