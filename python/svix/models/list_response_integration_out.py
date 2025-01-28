# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .integration_out import IntegrationOut


class ListResponseIntegrationOut(SvixBaseModel):
    data: t.List[IntegrationOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
