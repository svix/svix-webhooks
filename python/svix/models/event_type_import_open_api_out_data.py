# this file is @generated
import typing as t

from .common import BaseModel
from .event_type_from_open_api import EventTypeFromOpenApi


class EventTypeImportOpenApiOutData(BaseModel):
    modified: t.List[str]

    to_modify: t.Optional[t.List[EventTypeFromOpenApi]] = None
