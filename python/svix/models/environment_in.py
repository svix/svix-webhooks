# this file is @generated
import typing as t

from .common import BaseModel
from .event_type_in import EventTypeIn
from .template_in import TemplateIn


class EnvironmentIn(BaseModel):
    event_types: t.Optional[t.List[EventTypeIn]] = None

    settings: t.Optional[t.Dict[str, t.Any]] = None

    transformation_templates: t.Optional[t.List[TemplateIn]] = None
