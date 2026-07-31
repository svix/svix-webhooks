# this file is @generated
import typing as t

from .common import BaseModel
from .connector_out import ConnectorOut


class ListResponseConnectorOut(BaseModel):
    data: t.List[ConnectorOut]

    iterator: t.Optional[str]

    prev_iterator: t.Optional[str] = None

    done: bool
