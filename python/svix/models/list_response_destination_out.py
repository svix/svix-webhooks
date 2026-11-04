# this file is @generated
import typing as t

from .common import BaseModel
from .destination_out import DestinationOut


class ListResponseDestinationOut(BaseModel):
    data: t.List[DestinationOut]

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None

    done: bool
