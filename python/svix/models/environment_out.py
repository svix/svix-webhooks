# this file is @generated
import typing as t
from datetime import datetime

from .common import BaseModel
from .event_type_out import EventTypeOut
from .template_out import TemplateOut


class EnvironmentOut(BaseModel):
    created_at: datetime

    event_types: t.List[EventTypeOut]

    settings: t.Optional[t.Dict[str, t.Any]]

    transformation_templates: t.List[TemplateOut]

    version: t.Optional[int] = None
