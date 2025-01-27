# this file is @generated
import typing as t
from datetime import datetime

from .common import SvixBaseModel
from .event_type_out import EventTypeOut
from .template_out import TemplateOut


class EnvironmentOut(SvixBaseModel):
    created_at: datetime
    event_types: t.List[EventTypeOut]
    settings: t.Dict[str, t.Any]
    transformation_templates: t.List[TemplateOut]
    version: t.Optional[int] = None
