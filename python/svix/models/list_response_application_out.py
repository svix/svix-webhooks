# this file is @generated
import typing as t

from pydantic import Field

from .application_out import ApplicationOut
from .common import SvixBaseModel


class ListResponseApplicationOut(SvixBaseModel):
    data: t.List[ApplicationOut]

    done: bool

    iterator: str

    prev_iterator: t.Optional[str] = Field(default=None, alias="prevIterator")
