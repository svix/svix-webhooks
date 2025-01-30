# this file is @generated
import typing as t

from .application_out import ApplicationOut
from .common import BaseModel


class ListResponseApplicationOut(BaseModel):
    data: t.List[ApplicationOut]

    done: bool

    iterator: t.Optional[str] = None

    prev_iterator: t.Optional[str] = None
