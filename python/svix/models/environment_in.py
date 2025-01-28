# this file is @generated
import typing as t

from pydantic import Field

from .common import SvixBaseModel
from .event_type_in import EventTypeIn
from .template_in import TemplateIn


class EnvironmentIn(SvixBaseModel):
    event_types: t.Optional[t.List[EventTypeIn]] = Field(
        default=None, alias="eventTypes"
    )

    settings: t.Optional[t.Dict[str, t.Any]] = None

    transformation_templates: t.Optional[t.List[TemplateIn]] = Field(
        default=None, alias="transformationTemplates"
    )
