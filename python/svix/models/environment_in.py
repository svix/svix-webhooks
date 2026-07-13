# this file is @generated
import typing as t

from .common import BaseModel
from .connector_in import ConnectorIn
from .event_type_in import EventTypeIn


class EnvironmentIn(BaseModel):
    event_types: t.Optional[t.List[EventTypeIn]] = None

    settings: t.Optional[t.Dict[str, t.Any]] = None

    connectors: t.Optional[t.List[ConnectorIn]] = None
