# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .connector_out import ConnectorOut
from .event_type_out import EventTypeOut


class EnvironmentOut(BaseModel):
    created_at: datetime

    event_types: t.List[EventTypeOut]

    settings: t.Optional[t.Dict[str, t.Any]]

    transformation_templates: t.List[ConnectorOut]

    version: t.Optional[int] = None
