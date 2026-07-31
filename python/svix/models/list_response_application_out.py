# this file is @generated
import typing as t

from .application_out import ApplicationOut
from .common import BaseModel


class ListResponseApplicationOut(BaseModel):
    data: t.List[ApplicationOut]

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None

    done: bool
